extern crate hyper_tls;
extern crate iso_currency;
extern crate isocountry;
extern crate isolanguage_1;
extern crate log;
extern crate url;
pub mod chatex;
pub mod coin;
pub mod endpoint;
pub mod extractor;
pub mod models;

struct ClientBase<TConnector> {
    client: hyper::Client<TConnector>,
    api_context: chatex::ApiContext,
    access_controller: AccessController,
}

impl<TConnector> ClientBase<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    pub fn new(
        client: hyper::Client<TConnector>,
        api_context: chatex::ApiContext,
        access_controller: AccessController,
    ) -> ClientBase<TConnector> {
        ClientBase {
            client,
            api_context,
            access_controller,
        }
    }

    async fn get_access_token(&self) -> Option<chatex::AccessToken> {
        self.access_controller
            .get_access_token(&self.api_context, &self.client)
            .await
    }
}

pub struct ChatexClient<TConnector> {
    base: ClientBase<TConnector>,
    profile: endpoint::Profile,
    coin: endpoint::Coin,
}

impl<TConnector> ChatexClient<TConnector>
where
    TConnector: hyper::client::connect::Connect + Clone + Send + Sync + 'static,
{
    pub fn new(
        connector: TConnector,
        base_url: url::Url,
        secret: String,
    ) -> ChatexClient<TConnector>
    where
        TConnector: hyper::client::connect::Connect + Clone + Send + Sync + 'static,
    {
        let client = hyper::Client::builder().build::<TConnector, hyper::Body>(connector);
        let base_context = chatex::BaseContext::new(base_url);
        let api_context = chatex::ApiContext::new(base_context.clone(), secret);
        let profile = endpoint::Profile::new(&base_context);
        let coin = endpoint::Coin::new(&base_context);
        let access_controller = AccessController::new(profile.clone());
        let base = ClientBase::new(client, api_context, access_controller);
        ChatexClient {
            base,
            profile,
            coin,
        }
    }

    pub fn profile<'a: 'b, 'b>(&'a self) -> ProfileClient<'b, TConnector> {
        ProfileClient::new(&self.base, &self.profile)
    }

    pub fn coin<'a: 'b, 'b>(&'a self) -> CoinClient<'b, TConnector> {
        CoinClient::new(&self.base, &self.coin)
    }
}

struct AccessController {
    access_context: std::cell::RefCell<Option<chatex::AccessContext>>,
    profile: endpoint::Profile,
}

impl AccessController {
    pub fn new(profile: endpoint::Profile) -> AccessController {
        AccessController {
            access_context: std::cell::RefCell::new(None),
            profile,
        }
    }

    pub async fn get_access_token<TConnector>(
        &self,
        api_context: &chatex::ApiContext,
        client: &hyper::Client<TConnector>,
    ) -> Option<String>
    where
        TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
    {
        if self.access_context.borrow().is_none() {
            let auth_request = self
                .profile
                .get_access_token(api_context)
                .expect("Failed to craete access_token request!");
            let auth_response = client.request(auth_request).await.unwrap();
            if auth_response.status().is_success() {
                let auth_body = auth_response.into_body();
                let access_token = extractor::extract_access_token(auth_body)
                    .await
                    .expect("Failed to read the body of access token!");
                self.access_context.replace(Some(chatex::AccessContext::new(
                    api_context.base.clone(),
                    access_token.access_token,
                )));
            }
        }
        self.access_context.borrow().as_ref().map_or_else(
            || None,
            |access_context| Some(access_context.access_token.clone()),
        )
    }
}

pub struct ProfileClient<'a, TConnector> {
    base: &'a ClientBase<TConnector>,
    profile: &'a endpoint::Profile,
}

impl<'a, TConnector> ProfileClient<'a, TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    fn new(
        base: &'a ClientBase<TConnector>,
        profile: &'a endpoint::Profile,
    ) -> ProfileClient<'a, TConnector> {
        ProfileClient { base, profile }
    }

    pub async fn create_access_token(&self) -> Option<models::AccessToken> {
        let auth_request = self
            .profile
            .get_access_token(&self.base.api_context)
            .expect("Failed to craete access_token request!");
        let auth_response = self.base.client.request(auth_request).await.unwrap();
        if auth_response.status().is_success() {
            let auth_body = auth_response.into_body();
            extractor::extract_access_token(auth_body).await
        } else {
            None
        }
    }

    pub async fn get_account_information(&self) -> Option<models::BasicInfo> {
        if let Some(access_token) = self.base.get_access_token().await {
            let me_request = self
                .profile
                .get_me(&access_token)
                .expect("Failed to build /me request!");
            let me_response = self.base.client.request(me_request).await.ok()?;
            let me_body = me_response.into_body();
            extractor::extract_basic_info(me_body).await
        } else {
            None
        }
    }

    pub async fn get_balance_summary(&self) -> Option<models::Balance> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .profile
                .get_balance(&access_token)
                .expect("Failed to build /balance request!");
            let response = self.base.client.request(request).await.ok()?;
            let body = response.into_body();
            extractor::extract_balance(body).await
        } else {
            None
        }
    }
}

pub struct CoinClient<'a, TConnector> {
    base: &'a ClientBase<TConnector>,
    coin: &'a endpoint::Coin,
}

impl<'a, TConnector> CoinClient<'a, TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    fn new(
        base: &'a ClientBase<TConnector>,
        coin: &'a endpoint::Coin,
    ) -> CoinClient<'a, TConnector> {
        CoinClient { base, coin }
    }

    pub async fn get_available_coins(&self) -> Option<models::Coins> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .coin
                .coins(&access_token)
                .expect("Failed to build /coins request!");
            let response = self.base.client.request(request).await.ok()?;
            let body = response.into_body();
            extractor::extract_coins(body).await
        } else {
            None
        }
    }

    pub async fn get_coin(&self, coin: coin::Coin) -> Option<models::Coin> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .coin
                .coin(coin, &access_token)
                .expect("Failed to build /coins/name request!");
            let response = self.base.client.request(request).await.ok()?;
            let body = response.into_body();
            extractor::extract_coin(body).await
        } else {
            None
        }
    }
}
