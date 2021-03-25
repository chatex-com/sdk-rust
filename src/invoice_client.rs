use super::{client_base, coin, endpoint, error, extractor, models};
use chrono;
use hyper;
use iso_currency;
use isolanguage_1;

pub struct InvoiceClient<TConnector> {
    base: std::sync::Arc<client_base::ClientBase<TConnector>>,
    invoice: std::sync::Arc<endpoint::Invoice>,
}

impl<TConnector> InvoiceClient<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    pub fn new(
        base: std::sync::Arc<client_base::ClientBase<TConnector>>,
        invoice: std::sync::Arc<endpoint::Invoice>,
    ) -> InvoiceClient<TConnector> {
        InvoiceClient { base, invoice }
    }

    pub async fn get_invoices(
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
    ) -> Result<models::Invoices, error::Error> {
        let request = self
            .base
            .create_request(self.invoice.as_ref(), |access_token, invoice| {
                invoice
                    .get_invoices(
                        coins,
                        fiat,
                        country_code,
                        payment_system_id,
                        lang_id,
                        status,
                        offset,
                        limit,
                        date_start,
                        date_end,
                        &access_token,
                    )
                    .expect("Failed to build /invoices request!")
            });
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| extractor::extract_invoices(body))
                    .await
            }
            Err(error) => Err(error),
        }
    }

    pub async fn create_invoice(
        &self,
        create_invoice: models::CreateInvoice,
    ) -> Result<models::Invoices, error::Error> {
        let request = self
            .base
            .create_request(self.invoice.as_ref(), |access_token, invoice| {
                invoice
                    .create_invoice(create_invoice.clone(), &access_token)
                    .expect("Failed to build /invoices request!")
            });
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| extractor::extract_invoices(body))
                    .await
            }
            Err(error) => Err(error),
        }
    }

    pub async fn get_invoice_by_id(
        &self,
        id: &str,
    ) -> Result<models::Invoice, error::Error> {
        let request = self
            .base
            .create_request(self.invoice.as_ref(), |access_token, invoice| {
                invoice
                    .get_invoice_by_id(id.into(), &access_token)
                    .expect("Failed to build /invoices/id request!")
            });
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| extractor::extract_invoice(body))
                    .await
            }
            Err(error) => Err(error),
        }
    }
}
