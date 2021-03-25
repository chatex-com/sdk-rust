use hyper;
use serde;

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum Error {
    InternalServerError,
    NotFoundError,
    PermissionDeniedError,
    RateLimitedError { 
        #[serde(rename = "retryAfter")]
        retry_after: i64 
    },
    UnprocessableEntityError,
    ValidationError,
    BadRequest,
    Unauthorized,
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InternalServerError => {
                write!(formatter, "Internal Server Error")
            },
            Error::NotFoundError => {
                write!(formatter, "Not Found Error")
            },
            Error::PermissionDeniedError => {
                write!(formatter, "Permission Denied Error")
            },
            Error::RateLimitedError { retry_after } => {
                write!(formatter, "Rate Limited Error (retry after {} seconds)", retry_after)
            },
            Error::UnprocessableEntityError => { 
                write!(formatter,"Unprocessable Entity Error")
            },
            Error::ValidationError => {
                write!(formatter, "Validation Error")
            },
            Error::BadRequest => {
                write!(formatter, "Bad Request")
            },
            Error::Unauthorized => {
                write!(formatter, "Unauthorized. Invalid token.")
            },
        }
    }
}

impl std::error::Error for Error { }

impl Error {
    pub fn is_error_code(
        status_code: hyper::StatusCode,
    )-> bool {
        use hyper::StatusCode;
        match status_code {
            StatusCode::OK => false,
            StatusCode::CREATED => false,
            _ => true,
        }
    }

    pub async fn to_error(
        status_code: hyper::StatusCode,
        body: hyper::Body,
    ) -> Error {
        use hyper::StatusCode;
        use super::extractor;
        match status_code {
            StatusCode::BAD_REQUEST => Error::BadRequest,
            StatusCode::UNAUTHORIZED => {
                Error::Unauthorized
            },
            StatusCode::FORBIDDEN => {
                Error::PermissionDeniedError
            },
            StatusCode::NOT_FOUND => {
                Error::NotFoundError
            },
            StatusCode::UNPROCESSABLE_ENTITY => {
                Error::UnprocessableEntityError
            },
            StatusCode::TOO_MANY_REQUESTS => {
                extractor::read_body::<Error>(body)
                    .await
                    .map_or(Error::InternalServerError, |error| error)
            },
            _ => Error::InternalServerError,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use hyper::StatusCode;
    use tokio_test;

    #[test]
    fn is_error_code() {
        let true_status_codes = [
            StatusCode::OK,
            StatusCode::CREATED,
        ];
        let false_status_codes = [
            StatusCode::BAD_REQUEST,
            StatusCode::UNAUTHORIZED,
            StatusCode::FORBIDDEN,
            StatusCode::NOT_FOUND,
            StatusCode::UNPROCESSABLE_ENTITY,
            StatusCode::TOO_MANY_REQUESTS,
            StatusCode::NON_AUTHORITATIVE_INFORMATION,
            StatusCode::MULTI_STATUS,
        ];
        true_status_codes
            .iter()
            .for_each(|value| {
                let result = Error::is_error_code(value.clone());
                assert!(!result, "Expected this status code is not error code")
            });
        false_status_codes
            .iter()
            .for_each(|value| {
                let result = Error::is_error_code(value.clone());
                assert!(result, "Expected this status code is error code")
            });
    }

    #[test]
    fn to_error() {
        fn create_body(body_content: &'static str) -> hyper::Body {
            let (mut sender, body) = hyper::Body::channel();
            tokio_test::block_on(
                sender.send_data(hyper::body::Bytes::from(body_content)))
                .expect("Failed to send data");
            body
        }
        let empty_body = "{}";
        let body = create_body(empty_body);
        let status_code = StatusCode::BAD_REQUEST;
        let error = tokio_test::block_on(Error::to_error(status_code, body));
        match error {
            Error::BadRequest => assert!(true),
            _ => assert!(false, "Failed to get error code from empty enum!"),
        }
        let body = create_body(empty_body);
        let status_code = StatusCode::FORBIDDEN;
        let error = tokio_test::block_on(Error::to_error(status_code, body));
        match error {
            Error::PermissionDeniedError => assert!(true),
            _ => assert!(false, "Failed to get error code from empty enum!"),
        }
        let body = create_body(r#"{ "retryAfter": 2020 }"#);
        let status_code = StatusCode::TOO_MANY_REQUESTS;
        let error = tokio_test::block_on(Error::to_error(status_code, body));
        match error {
            Error::RateLimitedError { retry_after } => {
                assert_eq!(retry_after, 2020);
            },
            _ => assert!(false, "Failed to get error code from empty enum!"),
        }
    }
}
