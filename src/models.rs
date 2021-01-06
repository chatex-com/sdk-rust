#[derive(serde::Deserialize, Debug)]
pub struct AccessToken {
    pub access_token: String,
    pub expires_at: i32
}

#[derive(serde::Deserialize, Debug)]
pub struct BasicInfo {
    pub id: i64,
    pub merchant_info: Option<MerchantInfo>,
    pub profile: Profile
}

#[derive(serde::Deserialize, Debug)]
pub struct MerchantInfo {
    pub name: String,
    pub usd_amount_max_limit: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Profile {
    pub country_code: String,
    pub email: Option<String>,
    pub is_finance_blocked: bool,
    pub lang_id: String,
    pub limits: AML5Limits,
    pub phone: String,
    pub username: String,
    pub verification: Verification,
}

#[derive(serde::Deserialize, Debug)]
pub struct AML5Limits {
    pub current_turnover: String,
    pub current_withdraw: String,
    pub turnover_limit: String,
    pub withdraw_limit: String,
    pub withdraw_limit_daily: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Verification {
    pub current_level: String
}

pub type Balance = Vec<Currency>;

#[derive(serde::Deserialize, Debug)]
pub struct Currency {
    pub amount: String,
    pub coin: String,
    pub held: String,
}

pub type Coins = Vec<Coin>;

#[derive(serde::Deserialize, Debug)]
pub struct Coin {
    pub decimals: u32,
    pub full_name: String,
    pub name: String
}
