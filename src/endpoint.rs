use super::coin;
use super::context;
use super::models;
use chrono;
use iso_currency;
use isocountry;
use isolanguage_1;
use url;

#[derive(Clone, Debug)]
pub struct Profile {
    auth: url::Url,
    me: url::Url,
    balance: url::Url,
}

impl Profile {
    pub fn new(base_context: &context::BaseContext) -> Profile {
        let mut auth = base_context.base_url.clone();
        auth.path_segments_mut()
            .unwrap()
            .push("auth")
            .push("access-token");
        let mut me = base_context.base_url.clone();
        me.path_segments_mut().unwrap().push("me");
        let mut balance = base_context.base_url.clone();
        balance
            .path_segments_mut()
            .unwrap()
            .push("me")
            .push("balance");
        Profile { auth, me, balance }
    }

    pub fn get_access_token(
        &self,
        api_context: &context::ApiContext,
    ) -> Option<http::Request<hyper::Body>> {
        create_post_request_with_url(&api_context.api_key, &self.auth)
    }

    pub fn get_me(
        &self,
        access_token: &context::AccessToken,
    ) -> Option<http::Request<hyper::Body>> {
        create_get_request_with_url(&access_token, &self.me)
    }

    pub fn get_balance(&self, access_token: &str) -> Option<http::Request<hyper::Body>> {
        create_get_request_with_url(&access_token, &self.balance)
    }
}

#[derive(Clone, Debug)]
pub struct Coin {
    coins: url::Url,
}

impl Coin {
    pub fn new(base_context: &context::BaseContext) -> Coin {
        let mut coins = base_context.base_url.clone();
        coins.path_segments_mut().unwrap().push("coins");
        Coin { coins }
    }

    pub fn coins(&self, access_token: &str) -> Option<http::Request<hyper::Body>> {
        create_get_request_with_url(&access_token, &self.coins)
    }

    pub fn coin(
        &self,
        coin: super::coin::Coin,
        access_token: &context::AccessToken,
    ) -> Option<http::Request<hyper::Body>> {
        // Bad solution. There should be way to implement in without allocations.
        // The simplest way is to use somehow 'static str.
        let coin: String = coin.into();
        let mut url = self.coins.clone();
        url.path_segments_mut().unwrap().push(coin.as_ref());
        create_get_request_with_url(&access_token, &url)
    }
}

#[derive(Clone, Debug)]
pub struct Exchange {
    orders: url::Url,
    my: url::Url,
    trades: url::Url,
}

impl Exchange {
    const EXCHANGE: &'static str = "exchange";
    const ORDERS: &'static str = "orders";
    const TRADES: &'static str = "trades";
    const MY: &'static str = "my";
    const ACTIVATE: &'static str = "activate";
    const DEACTIVATE: &'static str = "deactivate";

    pub fn new(base_context: &context::BaseContext) -> Exchange {
        let mut exchange_url = base_context.base_url.clone();
        exchange_url
            .path_segments_mut()
            .unwrap()
            .push(Self::EXCHANGE);
        let mut orders = exchange_url.clone();
        orders.path_segments_mut().unwrap().push(Self::ORDERS);
        let mut my = orders.clone();
        my.path_segments_mut().unwrap().push(Self::MY);
        let mut trades = orders.clone();
        trades.path_segments_mut().unwrap().push(Self::TRADES);
        Exchange { orders, my, trades }
    }

    pub fn get_orders(
        &self,
        pair: coin::CoinPair,
        offset: Option<u32>,
        limit: Option<u32>,
        access_token: &context::AccessToken,
    ) -> Option<http::Request<hyper::Body>> {
        let pair = String::from(pair);
        let mut orders_url = self.orders.clone();
        orders_url
            .query_pairs_mut()
            .append_pair("pair", pair.as_ref());
        let offset = if let Some(offset) = &offset {
            offset
        } else {
            &0
        };
        orders_url
            .query_pairs_mut()
            .append_pair("offset", &offset.to_string());
        let limit = if let Some(limit) = &limit { limit } else { &50 };
        orders_url
            .query_pairs_mut()
            .append_pair("limit", &limit.to_string());
        create_get_request_with_url(&access_token, &orders_url)
    }

    pub fn post_order(
        &self,
        pair: coin::CoinPair,
        amount: String,
        rate: String,
        access_token: &context::AccessToken,
    ) -> Option<http::Request<hyper::Body>> {
        let pair = String::from(pair);
        let order_request = models::OrderRequest { pair, amount, rate };
        log::info!("Order request: {:#?}", order_request);
        let order_request = serde_json::to_vec(&order_request).unwrap();
        create_post_request_builder_with_url(&access_token, &self.orders)
            .header("Content-Type", "application/json")
            .body(hyper::body::Body::from(order_request))
            .ok()
    }

    pub fn get_my_orders(
        &self,
        pair: Option<coin::CoinPair>,
        status: Option<String>,
        offset: Option<u32>,
        limit: Option<u32>,
        access_token: &context::AccessToken,
    ) -> Option<http::Request<hyper::Body>> {
        let mut url = self.my.clone();
        if let Some(pair) = pair {
            url.query_pairs_mut()
                .append_pair("pair", String::from(pair).as_ref());
        }
        if let Some(status) = status {
            url.query_pairs_mut().append_pair("status", status.as_ref());
        }
        add_offset_and_limit_parameters(&mut url, offset, limit);
        create_get_request_with_url(&access_token, &url)
    }

    pub fn get_trades(
        &self,
        order_id: Option<u32>,
        offset: Option<u32>,
        limit: Option<u32>,
        access_token: &context::AccessToken,
    ) -> Option<http::Request<hyper::Body>> {
        let mut url = self.trades.clone();
        if let Some(order_id) = order_id {
            url.query_pairs_mut()
                .append_pair("order_id", order_id.to_string().as_ref());
        }
        add_offset_and_limit_parameters(&mut url, offset, limit);
        create_get_request_with_url(&access_token, &url)
    }

    pub fn get_trade_by_id(
        &self,
        id: &str,
        access_token: &context::AccessToken,
    ) -> Option<http::Request<hyper::Body>> {
        let mut url = self.trades.clone();
        url.path_segments_mut().unwrap().push(id);
        create_get_request_with_url(&access_token, &url)
    }

    pub fn get_order_by_id(
        &self,
        id: &str,
        access_token: &context::AccessToken,
    ) -> Option<http::Request<hyper::Body>> {
        let mut url = self.orders.clone();
        url.path_segments_mut().unwrap().push(id);
        create_get_request_with_url(&access_token, &url)
    }

    pub fn update_order_by_id(
        &self,
        id: &str,
        order: models::UpdateOrder,
        access_token: &context::AccessToken,
    ) -> Option<http::Request<hyper::Body>> {
        let mut url = self.orders.clone();
        url.path_segments_mut().unwrap().push(id);
        let order = serde_json::to_vec(&order).unwrap();
        create_default_request_builder(&access_token)
            .method(hyper::Method::PUT)
            .uri(url.to_string())
            .header("Content-Type", "application/json")
            .body(hyper::Body::from(order))
            .ok()
    }

    pub fn delete_order_by_id(
        &self,
        id: &str,
        access_token: &context::AccessToken,
    ) -> Option<http::Request<hyper::Body>> {
        let mut url = self.orders.clone();
        url.path_segments_mut().unwrap().push(id);
        create_default_request_builder(&access_token)
            .method(hyper::Method::DELETE)
            .uri(url.to_string())
            .body(hyper::Body::empty())
            .ok()
    }

    pub fn activate_order_by_id(
        &self,
        id: &str,
        access_token: &context::AccessToken,
    ) -> Option<http::Request<hyper::Body>> {
        let mut url = self.orders.clone();
        url.path_segments_mut()
            .unwrap()
            .push(id)
            .push(Self::ACTIVATE);
        create_post_request_with_url(&access_token, &url)
    }

    pub fn deactivate_order_by_id(
        &self,
        id: &str,
        access_token: &context::AccessToken,
    ) -> Option<http::Request<hyper::Body>> {
        let mut url = self.orders.clone();
        url.path_segments_mut()
            .unwrap()
            .push(id)
            .push(Self::DEACTIVATE);
        create_post_request_with_url(&access_token, &url)
    }

    pub fn create_trade_for_order(
        &self,
        id: &str,
        trade: &models::CreateTradeRequest,
        access_token: &context::AccessToken,
    ) -> Option<http::Request<hyper::Body>> {
        let mut url = self.orders.clone();
        url.path_segments_mut().unwrap().push(id).push(Self::TRADES);
        log::info!("Url: {}, Body: {:#?}", url, serde_json::to_string(trade));
        let trade = serde_json::to_vec(trade).unwrap();
        create_post_request_builder_with_url(&access_token, &url)
            .header("Content-Type", "application/json")
            .body(hyper::Body::from(trade))
            .ok()
    }
}

#[derive(Clone, Debug)]
pub struct Invoice {
    invoices: url::Url,
}

impl Invoice {
    const INVOICES: &'static str = "invoices";

    pub fn new(base_context: &context::BaseContext) -> Invoice {
        let mut invoices = base_context.base_url.clone();
        invoices.path_segments_mut().unwrap().push(Self::INVOICES);
        Invoice { invoices }
    }

    pub fn get_invoices(
        &self,
        coins: Option<&[coin::Coin]>,
        fiat: Option<&[iso_currency::Currency]>,
        country_code: Option<&[isocountry::CountryCode]>,
        payment_system_id: Option<&[models::PaymentSystemId]>,
        lang_id: Option<&[isolanguage_1::LanguageCode]>,
        status: Option<&[models::InvoiceStatus]>,
        offset: Option<u32>,
        limit: Option<u32>,
        date_start: Option<chrono::DateTime<chrono::Utc>>,
        date_end: Option<chrono::DateTime<chrono::Utc>>,
        access_token: &context::AccessToken,
    ) -> Option<http::Request<hyper::Body>> {
        let mut url = self.invoices.clone();
        if let Some(coins) = coins {
            let coins = Self::slice_to_string(coins);
            url.query_pairs_mut().append_pair("coins", coins.as_ref());
        }
        if let Some(fiat) = fiat {
            let fiat = Self::slice_to_string(fiat);
            url.query_pairs_mut().append_pair("fiat", fiat.as_ref());
        }
        if let Some(country_code) = country_code {
            let country_code = Self::slice_to_string(country_code);
            url.query_pairs_mut()
                .append_pair("country_code", country_code.as_ref());
        }
        if let Some(payment_system_id) = payment_system_id {
            let payment_system_id = Self::slice_to_string(payment_system_id);
            url.query_pairs_mut()
                .append_pair("payment_system_id", payment_system_id.as_ref());
        }
        if let Some(lang_id) = lang_id {
            let lang_id = Self::slice_to_string(lang_id);
            url.query_pairs_mut()
                .append_pair("lang_id", lang_id.as_ref());
        }
        if let Some(status) = status {
            Self::slice_to_string(status);
        }
        add_offset_and_limit_parameters(&mut url, offset, limit);
        if let Some(date_start) = date_start {
            let date_start = format!("{}", date_start);
            url.query_pairs_mut()
                .append_pair("date_start", date_start.as_ref());
        }
        if let Some(date_end) = date_end {
            let date_end = format!("{}", date_end);
            url.query_pairs_mut()
                .append_pair("date_end", date_end.as_ref());
        }
        create_get_request_with_url(&access_token, &url)
    }

    pub fn create_invoice(
        &self,
        invoice: models::CreateInvoice,
        access_token: &context::AccessToken,
    ) -> Option<http::Request<hyper::Body>> {
        let url = self.invoices.clone();
        let invoice = serde_json::to_vec(&invoice).unwrap();
        create_post_request_builder_with_url(&access_token, &url)
            .header("Content-Type", "application/json")
            .body(hyper::Body::from(invoice))
            .ok()
    }

    pub fn get_invoice_by_id(
        &self,
        id: String,
        access_token: &context::AccessToken,
    ) -> Option<http::Request<hyper::Body>> {
        let mut url = self.invoices.clone();
        url.path_segments_mut().unwrap().push(id.as_ref());
        create_get_request_with_url(&access_token, &url)
    }

    fn slice_to_string<T>(slice: &[T]) -> String
    where
        T: std::fmt::Display,
    {
        let mut result = String::with_capacity(slice.len() * 4);
        for item in slice.iter() {
            result.push_str(format!("{}", item).as_ref());
        }
        result
    }
}

#[derive(Clone, Debug)]
pub struct PaymentSystem {
    payment_systems: url::Url,
    estimate: url::Url,
}

impl PaymentSystem {
    const PAYMENT_SYSTEMS: &'static str = "payment-systems";
    const ESTIMATE: &'static str = "estimate";

    pub fn new(base_context: &context::BaseContext) -> PaymentSystem {
        let mut payment_systems = base_context.base_url.clone();
        payment_systems
            .path_segments_mut()
            .unwrap()
            .push(Self::PAYMENT_SYSTEMS);
        let mut estimate = payment_systems.clone();
        estimate.path_segments_mut().unwrap().push(Self::ESTIMATE);
        PaymentSystem {
            payment_systems,
            estimate,
        }
    }

    pub fn get_list_of_estimated_payment_systems(
        &self,
        estimate: models::Estimate,
        access_token: &context::AccessToken,
    ) -> Option<http::Request<hyper::Body>> {
        let url = self.estimate.clone();
        let estimate = serde_json::to_vec(&estimate).unwrap();
        create_post_request_builder_with_url(&access_token, &url)
            .header("Content-Type", "application/json")
            .body(hyper::Body::from(estimate))
            .ok()
    }

    pub fn get_payment_system_by_id(
        &self,
        id: models::PaymentSystemId,
        access_token: &context::AccessToken,
    ) -> Option<http::Request<hyper::Body>> {
        let mut url = self.payment_systems.clone();
        url.path_segments_mut()
            .unwrap()
            .push(id.to_string().as_ref());
        create_get_request_with_url(&access_token, &url)
    }
}

fn create_default_request_builder(token: &str) -> http::request::Builder {
    http::request::Builder::new()
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", &token))
}

fn create_get_request_with_url(
    token: &str,
    url: &url::Url,
) -> Option<http::Request<hyper::Body>> {
    create_default_request_builder(token)
        .method(hyper::Method::GET)
        .uri(url.to_string())
        .body(hyper::Body::empty())
        .ok()
}

fn create_post_request_builder_with_url(
    token: &str,
    url: &url::Url,
) -> http::request::Builder {
    create_default_request_builder(token)
        .method(hyper::Method::POST)
        .uri(url.to_string())
}

fn create_post_request_with_url(
    token: &str,
    url: &url::Url,
) -> Option<http::Request<hyper::Body>> {
    create_post_request_builder_with_url(token, url)
        .body(hyper::Body::empty())
        .ok()
}

fn add_offset_and_limit_parameters(
    url: &mut url::Url,
    offset: Option<u32>,
    limit: Option<u32>,
) {
    let offset = if let Some(offset) = offset { offset } else { 0 };
    let limit = if let Some(limit) = limit { limit } else { 50 };
    url.query_pairs_mut()
        .append_pair("offset", offset.to_string().as_ref())
        .append_pair("limit", limit.to_string().as_ref());
}
