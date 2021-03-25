use chatex_sdk_rust;
use hyper_tls;
use simple_log;
use dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    simple_log::quick().ok();
    dotenv::dotenv().ok();
    let base_url = std::env::var("BASE_URL")
        .expect("Failed to get BASE_URL variable")
        .parse::<url::Url>()
        .expect("Failed to parse url");
    let secret = std::env::var("API_KEY")
        .expect("Failed to get API_KEY variable");
    let https = hyper_tls::HttpsConnector::new();
    let chatex = chatex_sdk_rust::ChatexClient::new(
        https,
        base_url,
        secret);
    let basic_info = chatex.profile().get_account_information().await;
    println!("Basic info: {:?}", basic_info);
    Ok(())
}
