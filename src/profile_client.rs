use super::client_base;
use super::endpoint;
use super::error;
use super::extractor;
use super::models;
use hyper;

pub struct ProfileClient<TConnector> {
    base: std::sync::Arc<client_base::ClientBase<TConnector>>,
    profile: std::sync::Arc<endpoint::Profile>,
}

impl<TConnector> ProfileClient<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    pub fn new(
        base: std::sync::Arc<client_base::ClientBase<TConnector>>,
        profile: std::sync::Arc<endpoint::Profile>,
    ) -> ProfileClient<TConnector> {
        ProfileClient { base, profile }
    }

    pub async fn create_access_token(&self) -> Result<models::AccessToken, error::Error> {
        let auth_request = self
            .profile
            .get_access_token(&self.base.api_context)
            .expect("Failed to create access_token request!");
        let auth_response = self.base.client.request(auth_request).await.unwrap();
        if error::Error::is_error_code(auth_response.status()) {
            let error =
                error::Error::to_error(auth_response.status(), auth_response.into_body())
                    .await;
            Err(error)
        } else {
            let auth_body = auth_response.into_body();
            Ok(extractor::extract_access_token(auth_body).await.unwrap())
        }
    }

    pub async fn get_account_information(
        &self,
    ) -> Result<models::BasicInfo, error::Error> {
        let request =
            self.base
                .create_request(self.profile.as_ref(), |access_token, profile| {
                    profile
                        .get_me(&access_token)
                        .expect("Failed to build /me request")
                });
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| extractor::extract_basic_info(body))
                    .await
            }
            Err(error) => Err(error),
        }
    }

    pub async fn get_balance_summary(&self) -> Result<models::Balance, error::Error> {
        let request =
            self.base
                .create_request(self.profile.as_ref(), |access_token, profile| {
                    profile
                        .get_balance(&access_token)
                        .expect("Failed to build /balance request")
                });
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| extractor::extract_balance(body))
                    .await
            }
            Err(error) => Err(error),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::test::*;

    fn create_profile_client(test_case: &TestCase) -> super::ProfileClient<Connector> {
        let profile = crate::endpoint::Profile::new(&test_case.base_context);
        let profile = std::sync::Arc::new(profile);
        super::ProfileClient::new(test_case.client_base.clone(), profile)
    }

    #[test]
    fn access_token() {
        let test_case = TestCase::new();
        let access_token_mock = test_case.mock_access_token();
        let profile_client = create_profile_client(&test_case);
        let access_token = profile_client.create_access_token();
        let access_token = tokio_test::block_on(access_token).unwrap();
        println!("{:#?}", access_token);
        access_token_mock.assert();
    }

    #[test]
    fn basic_info() {
        let test_case = TestCase::new();
        let access_token_mock = test_case.mock_access_token();

        let basic_info = serde_json::to_string(&crate::models::BasicInfo::default())
            .expect(SERDE_ERROR);
        let me_mock = test_case.server.mock(|when, then| {
            default_get_when(when)
                .path("/me");
            default_then_content_type(then)
                .status(200)
                .body(basic_info.clone());
        });
        let profile_client = create_profile_client(&test_case);
        let account_information = profile_client.get_account_information();
        let account_information = tokio_test::block_on(account_information);
        println!("{}", basic_info);
        println!("{:#?}", account_information);
        let account_information =
            serde_json::to_string(&account_information.unwrap()).expect(SERDE_ERROR);
        assert_eq!(account_information, basic_info);
        access_token_mock.assert();
        me_mock.assert();
    }

    #[test]
    fn balance() {
        let test_case = TestCase::new();
        let access_token_mock = test_case.mock_access_token();
        let balance: crate::models::Balance =
            vec![Default::default(), Default::default(), Default::default()];
        let balance = serde_json::to_string(&balance).unwrap();
        let balance_mock = test_case.server.mock(|when, then| {
            default_get_when(when)
                .path("/me/balance");
            default_then_content_type(then)
                .status(200)
                .body(balance.clone());
        });
        let profile_client = create_profile_client(&test_case);
        let balance_summary = profile_client.get_balance_summary();
        let balance_summary = tokio_test::block_on(balance_summary);
        println!("{:#?}", balance_summary);
        let balance_summary =
            serde_json::to_string(&balance_summary.unwrap()).expect(SERDE_ERROR);
        assert_eq!(balance_summary, balance);
        access_token_mock.assert();
        balance_mock.assert();
    }
}
