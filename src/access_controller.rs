use super::context;
use super::endpoint;
use super::error;
use super::extractor;
use hyper;

pub struct AccessController {
    access_context: std::sync::RwLock<Option<context::AccessContext>>,
    profile: std::sync::Arc<endpoint::Profile>,
}

impl AccessController {
    pub fn new(profile: std::sync::Arc<endpoint::Profile>) -> AccessController {
        AccessController {
            access_context: std::sync::RwLock::new(None),
            profile,
        }
    }

    pub async fn get_access_token<TConnector>(
        &self,
        api_context: &context::ApiContext,
        client: &hyper::Client<TConnector>,
    ) -> Result<String, error::Error>
    where
        TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
    {
        if self.access_context.read().unwrap().is_none()
            || self
                .access_context
                .read()
                .unwrap()
                .as_ref()
                .unwrap()
                .expired()
        {
            log::debug!("Requesting new access token!");
            let auth_request = self
                .profile
                .get_access_token(api_context)
                .expect("Failed to create access_token request!");
            let auth_response = client.request(auth_request).await.unwrap();
            if error::Error::is_error_code(auth_response.status()) {
                let error = error::Error::to_error(
                    auth_response.status(),
                    auth_response.into_body(),
                )
                .await;
                return Err(error);
            } else {
                let auth_body = auth_response.into_body();
                let access_token = extractor::extract_access_token(auth_body)
                    .await
                    .expect("Failed to read the body of access token!");
                let mut context = self.access_context.write().unwrap();
                *context = Some(context::AccessContext::new(
                    api_context.base.clone(),
                    access_token,
                ));
            }
        }
        self.access_context.read().unwrap().as_ref().map_or_else(
            || Err(error::Error::InternalServerError),
            |access_context| Ok(access_context.access_token.access_token.clone()),
        )
    }
}
