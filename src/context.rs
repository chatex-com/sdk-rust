use super::models;
use chrono;
use url;

#[derive(Clone)]
pub struct BaseContext {
    pub base_url: url::Url,
}

impl BaseContext {
    pub fn new(base_url: url::Url) -> BaseContext {
        BaseContext { base_url }
    }
}

pub struct ApiContext {
    pub base: BaseContext,
    pub api_key: String,
}

impl ApiContext {
    pub fn new(base: BaseContext, api_key: String) -> ApiContext {
        ApiContext { base, api_key }
    }
}

pub type AccessToken = String;

pub struct AccessContext {
    pub base: BaseContext,
    pub access_token: models::AccessToken,
    expires_at: chrono::DateTime<chrono::Utc>,
}

impl AccessContext {
    const TIME_EXPIRATION_TOLERANCE: i64 = 60;

    pub fn new(base: BaseContext, access_token: models::AccessToken) -> AccessContext {
        let expires_at =
            chrono::NaiveDateTime::from_timestamp(access_token.expires_at, 0);
        let expires_at =
            chrono::DateTime::<chrono::Utc>::from_utc(expires_at, chrono::Utc);
        AccessContext {
            base,
            access_token,
            expires_at,
        }
    }

    pub fn expired(&self) -> bool {
        let subtracted = chrono::Utc::now().signed_duration_since(self.expires_at);
        subtracted.num_seconds() > -Self::TIME_EXPIRATION_TOLERANCE
    }

    pub fn not_expired(&self) -> bool {
        !self.expired()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_access_context(time: chrono::DateTime<chrono::Utc>) -> AccessContext {
        let blank_url = url::Url::parse("http://localhost:8000").unwrap();
        let base_context = BaseContext::new(blank_url);
        AccessContext::new(
            base_context,
            models::AccessToken {
                access_token: String::new(),
                expires_at: time.timestamp(),
            },
        )
    }

    #[test]
    fn test_expired() {
        let access_context = create_access_context(chrono::Utc::now());
        assert!(access_context.expired(), "AccessContext must be expired!");
    }

    #[test]
    fn test_not_expired() {
        let valid_time = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::seconds(
                AccessContext::TIME_EXPIRATION_TOLERANCE + 1,
            ))
            .expect("Failed to add TIME_EXPIRATION_TOLERANCE");
        let access_context = create_access_context(valid_time);
        assert!(
            access_context.not_expired(),
            "AccessContext must not be expired!"
        );
    }
}
