use super::{
    access_controller, client_base, coin_client, context, endpoint, exchange_client,
    invoice_client, payment_system_client, profile_client,
};
use hyper;

pub struct ChatexClient<TConnector> {
    base: std::sync::Arc<client_base::ClientBase<TConnector>>,
    profile: std::sync::Arc<endpoint::Profile>,
    coin: std::sync::Arc<endpoint::Coin>,
    exchange: std::sync::Arc<endpoint::Exchange>,
    invoice: std::sync::Arc<endpoint::Invoice>,
    payment_system: std::sync::Arc<endpoint::PaymentSystem>,
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
        let base_context = context::BaseContext::new(base_url);
        let api_context = context::ApiContext::new(base_context.clone(), secret);
        let profile = endpoint::Profile::new(&base_context);
        let profile = std::sync::Arc::new(profile);
        let coin = endpoint::Coin::new(&base_context);
        let coin = std::sync::Arc::new(coin);
        let exchange = endpoint::Exchange::new(&base_context);
        let exchange = std::sync::Arc::new(exchange);
        let invoice = endpoint::Invoice::new(&base_context);
        let invoice = std::sync::Arc::new(invoice);
        let payment_system = endpoint::PaymentSystem::new(&base_context);
        let payment_system = std::sync::Arc::new(payment_system);
        let access_controller = access_controller::AccessController::new(profile.clone());
        let base = client_base::ClientBase::new(client, api_context, access_controller);
        let base = std::sync::Arc::new(base);
        ChatexClient {
            base,
            profile,
            coin,
            exchange,
            invoice,
            payment_system,
        }
    }

    pub fn profile(&self) -> profile_client::ProfileClient<TConnector> {
        profile_client::ProfileClient::new(self.base.clone(), self.profile.clone())
    }

    pub fn coin(&self) -> coin_client::CoinClient<TConnector> {
        coin_client::CoinClient::new(self.base.clone(), self.coin.clone())
    }

    pub fn exchange(&self) -> exchange_client::ExchangeClient<TConnector> {
        exchange_client::ExchangeClient::new(self.base.clone(), self.exchange.clone())
    }

    pub fn invoice(&self) -> invoice_client::InvoiceClient<TConnector> {
        invoice_client::InvoiceClient::new(self.base.clone(), self.invoice.clone())
    }

    pub fn payment_system(
        &self,
    ) -> payment_system_client::PaymentSystemClient<TConnector> {
        payment_system_client::PaymentSystemClient::new(
            self.base.clone(),
            self.payment_system.clone(),
        )
    }
}
