use super::access_controller;
use super::context;
use super::error;
use hyper;
use http;
use futures;

pub struct ClientBase<TConnector> {
    pub client: hyper::Client<TConnector>,
    pub api_context: context::ApiContext,
    access_controller: access_controller::AccessController,
}

impl<TConnector> ClientBase<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    pub fn new(
        client: hyper::Client<TConnector>,
        api_context: context::ApiContext,
        access_controller: access_controller::AccessController,
    ) -> ClientBase<TConnector> {
        ClientBase {
            client,
            api_context,
            access_controller,
        }
    }

    pub async fn get_access_token(&self) -> Result<context::AccessToken, error::Error> {
        self.access_controller
            .get_access_token(&self.api_context, &self.client)
            .await
    }

    pub async fn create_request<Endpoint, CreateRequest>(
        &self,
        endpoint: &Endpoint,
        create_request: CreateRequest,
    ) -> Result<http::Request<hyper::Body>, error::Error> 
    where
        CreateRequest: Fn(context::AccessToken, &Endpoint) -> http::Request<hyper::Body>,
    {
        let access_token = match self.get_access_token().await {
            Ok(access_token) => access_token,
            Err(error) => return Err(error),
        };
        Ok(create_request(access_token, endpoint))
    }

    pub async fn call_to_endpoint<F, ProcessResponse, TResult>(
        &self,
        request: http::Request<hyper::Body>,
        process_response: ProcessResponse,
    ) -> Result<TResult, error::Error> 
    where 
        F: futures::Future<Output=Option<TResult>>,
        ProcessResponse: 'static + Fn(hyper::Body) -> F,
    {
        let (header, body) = match self.client.request(request).await {
            Ok(response) => {
                response.into_parts()
            },
            Err(error) => {
                log::error!("{}", error);
                return Err(error::Error::InternalServerError);
            },
        };
        if error::Error::is_error_code(header.status) {
            let error = error::Error::to_error(header.status, body).await;
            Err(error)
        } else {
            Ok(process_response(body).await.unwrap())
        }
    }
}
