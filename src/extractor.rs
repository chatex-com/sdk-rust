use super::models;
use hyper;
use serde;
use serde_json;

pub async fn extract_access_token(body: hyper::Body) -> Option<models::AccessToken> {
    read_body::<models::AccessToken>(body).await
}

pub async fn extract_basic_info(body: hyper::Body) -> Option<models::BasicInfo> {
    read_body::<models::BasicInfo>(body).await
}

pub async fn extract_balance(body: hyper::Body) -> Option<models::Balance> {
    read_body::<models::Balance>(body).await
}

pub async fn extract_coins(body: hyper::Body) -> Option<models::Coins> {
    read_body::<models::Coins>(body).await
}

pub async fn extract_coin(body: hyper::Body) -> Option<models::Coin> {
    read_body::<models::Coin>(body).await
}

pub async fn extract_orders(body: hyper::Body) -> Option<models::Orders> {
    read_body::<models::Orders>(body).await
}

pub async fn extract_order(body: hyper::Body) -> Option<models::Order> {
    read_body::<models::Order>(body).await
}

pub async fn extract_trades(body: hyper::Body) -> Option<models::Trades> {
    read_body::<models::Trades>(body).await
}

pub async fn extract_trade(body: hyper::Body) -> Option<models::Trade> {
    read_body::<models::Trade>(body).await
}

pub async fn extract_invoices(body: hyper::Body) -> Option<models::Invoices> {
    read_body::<models::Invoices>(body).await
}

pub async fn extract_invoice(body: hyper::Body) -> Option<models::Invoice> {
    read_body::<models::Invoice>(body).await
}

pub async fn extract_payment_system(body: hyper::Body) -> Option<models::PaymentSystem> {
    read_body::<models::PaymentSystem>(body).await
}

pub async fn extract_fiat_estimations(
    body: hyper::Body,
) -> Option<models::FiatEstimations> {
    read_body::<models::FiatEstimations>(body).await
}

pub async fn read_body<TResult>(body: hyper::body::Body) -> Option<TResult>
where
    TResult: serde::de::DeserializeOwned,
{
    let body = hyper::body::to_bytes(body).await.ok()?;
    match serde_json::from_slice(&body) {
        Ok(result) => Some(result),
        Err(error) => {
            eprintln!("Error on read_body: {:?}", error);
            None
        }
    }
}
