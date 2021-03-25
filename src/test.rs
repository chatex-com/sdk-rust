pub const SECRET: &'static str = "SECRET";
pub const SERDE_ERROR: &'static str = "Failed to serialize something.";

pub type Connector = hyper::client::HttpConnector;

pub struct TestCase {
    pub server: httpmock::MockServer,
    pub client_base: std::sync::Arc<crate::client_base::ClientBase<Connector>>,
    pub base_context: crate::context::BaseContext,
}

impl TestCase {
    pub fn new() -> Self {
        let server = httpmock::MockServer::start();
        let base_url = url::Url::parse(&server.base_url()).unwrap();
        let hyper_client = hyper::Client::builder()
            .build_http::<hyper::Body>();
        let base_context = crate::context::BaseContext::new(base_url);
        let api_context = crate::context::ApiContext::new(
            base_context.clone(),
            SECRET.to_owned());
        let profile = crate::endpoint::Profile::new(&base_context);
        let profile= std::sync::Arc::new(profile);
        let access_controller = crate::access_controller::AccessController::new(
            profile.clone());
        let client_base = std::sync::Arc::new(crate::client_base::ClientBase::new(
            hyper_client,
            api_context,
            access_controller));
        TestCase {
            server,
            client_base,
            base_context,
        }
    }

    pub fn mock_access_token(&self) -> httpmock::MockRef {
        self.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .header("Authorization", "Bearer SECRET")
                .header("Accept", "application/json")
                .path("/auth/access-token");
            let access_token = serde_json::to_string(
                &crate::models::AccessToken::default()).expect(SERDE_ERROR);
            default_then_content_type(then)
                .status(200)
                .body(access_token);
        })
    }
}

pub fn default_get_when(when: httpmock::When) -> httpmock::When {
    when
        .method(httpmock::Method::GET)
        .header("Accept", "application/json")
        .header("Authorization", "Bearer TOKEN")
}

pub fn default_post_when(when: httpmock::When) -> httpmock::When {
    default_when_content_type(when)
        .method(httpmock::Method::POST)
        .header("Accept", "application/json")
        .header("Authorization", "Bearer TOKEN")
}

pub fn default_then_content_type(then: httpmock::Then) -> httpmock::Then {
    then.header("Content-Type", "application/json")
}

pub fn default_when_content_type(when: httpmock::When) -> httpmock::When {
    when.header("Content-Type", "application/json")
}
