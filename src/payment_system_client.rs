use super::{client_base, endpoint, error, extractor, models};
use hyper;

pub struct PaymentSystemClient<TConnector> {
    base: std::sync::Arc<client_base::ClientBase<TConnector>>,
    payment_system: std::sync::Arc<endpoint::PaymentSystem>,
}

impl<'a, TConnector> PaymentSystemClient<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    pub fn new(
        base: std::sync::Arc<client_base::ClientBase<TConnector>>,
        payment_system: std::sync::Arc<endpoint::PaymentSystem>,
    ) -> PaymentSystemClient<TConnector> {
        PaymentSystemClient {
            base,
            payment_system,
        }
    }

    pub async fn get_list_of_estimated_payment_systems(
        &self,
        estimate: models::Estimate,
    ) -> Result<models::FiatEstimations, error::Error> {
        let request = self.base.create_request(
            self.payment_system.as_ref(),
            |access_token, payment_system| {
                payment_system
                    .get_list_of_estimated_payment_systems(
                        estimate.clone(),
                        &access_token,
                    )
                    .expect("Failed to build /payment-system/estimate request!")
            },
        );
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| {
                        extractor::extract_fiat_estimations(body)
                    })
                    .await
            }
            Err(error) => Err(error),
        }
    }

    pub async fn get_payment_system_by_id(
        &self,
        id: models::PaymentSystemId,
    ) -> Result<models::PaymentSystem, error::Error> {
        let request = self.base.create_request(
            self.payment_system.as_ref(),
            |access_token, payment_system| {
                payment_system
                    .get_payment_system_by_id(id, &access_token)
                    .expect("Failed to build /payment-system/id request!")
            },
        );
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| {
                        extractor::extract_payment_system(body)
                    })
                    .await
            }
            Err(error) => Err(error),
        }
    }
}
