use super::constants::HtmlPropertyNames;
use super::constants::HtmlTagNames;
use super::constants::InvoicePageNameValues;
use super::constants::IrsPtUrls;
use super::extensions::ElementProp;
use super::extensions::WebDriverExtensions;
use super::extensions::WebElementExtensions;
use super::IrsptApi;
use crate::models::IssueInvoiceRequest;
use crate::models::ReferenceDataDto;
use crate::traits::TIrsptApiInvoices;

use anyhow::Result;
use async_trait::async_trait;
use thirtyfour::prelude::ElementWaitable;
use thirtyfour::By;

#[async_trait]
impl<'a> TIrsptApiInvoices<'a> for IrsptApi {
    // TODO: Add tests.
    async fn issue_invoice_async(&self, request_model: &IssueInvoiceRequest) -> Result<()> {
        let is_portuguese_client = request_model.get_client_country() == "PORTUGAL";

        self.web_driver
            .goto(format!(
                "{}#?modoConsulta=Prestador&nifPrestadorServicos={}&isAutoSearchOn=on",
                IrsPtUrls::ISSUE_INVOICE_PAGE,
                request_model.get_nif()
            ))
            .await?;

        let _ = &self
            .web_driver
            .find_by_prop_value_async(
                HtmlTagNames::INPUT,
                HtmlPropertyNames::NAME,
                InvoicePageNameValues::SERVICE_DATE,
            )
            .await?
            .wait_until()
            .displayed()
            .await?;

        let _ = &self
            .web_driver
            .set_input_value_by_prop_value_async(
                HtmlPropertyNames::NAME,
                InvoicePageNameValues::SERVICE_DATE,
                request_model.get_date(),
            )
            .await?;

        let _ = &self
            .web_driver
            .find_by_prop_value_async(
                HtmlTagNames::SELECT,
                HtmlPropertyNames::NAME,
                InvoicePageNameValues::INVOICE_TYPE,
            )
            .await?
            .select_option_by_prop_value_async("label", "Fatura-Recibo")
            .await?;

        let _ = &self
            .web_driver
            .find_by_prop_value_async(
                HtmlTagNames::SELECT,
                HtmlPropertyNames::NAME,
                InvoicePageNameValues::CLIENT_COUNTRY,
            )
            .await?
            .wait_until()
            .displayed()
            .await?;

        let _ = &self
            .web_driver
            .find_by_prop_value_async(
                HtmlTagNames::SELECT,
                HtmlPropertyNames::NAME,
                InvoicePageNameValues::CLIENT_COUNTRY,
            )
            .await?
            .select_option_by_prop_value_containing_async(
                "label",
                request_model.get_client_country(),
            )
            .await?;

        let _ = &self
            .web_driver
            .set_input_value_by_prop_value_async(
                HtmlPropertyNames::NAME,
                if is_portuguese_client {
                    InvoicePageNameValues::VAT_NATIONAL_CLIENT
                } else {
                    InvoicePageNameValues::VAT_INTERNATIONAL_CLIENT
                },
                request_model.get_client_nif(),
            )
            .await?;

        let _ = &self
            .web_driver
            .set_input_value_by_prop_value_async(
                HtmlPropertyNames::NAME,
                InvoicePageNameValues::CLIENT_NAME,
                request_model.get_client_name(),
            )
            .await?;

        if is_portuguese_client {
            let _ = &self
                .web_driver
                .set_input_value_by_prop_value_async(
                    HtmlPropertyNames::NAME,
                    InvoicePageNameValues::CLIENT_ADDRESS,
                    request_model.get_client_address(),
                )
                .await;
        }

        let _ = &self
            .web_driver
            .find_by_props_value_async(
                HtmlTagNames::INPUT,
                &[
                    &ElementProp {
                        prop_name: HtmlPropertyNames::NAME,
                        prop_value: "titulo",
                    },
                    // TODO: Add support for different titles of payment ("títulos de pagamento") - by using .reference_data().
                    &ElementProp {
                        prop_name: HtmlPropertyNames::VALUE,
                        prop_value: "1",
                    },
                ],
            )
            .await?
            .click()
            .await?;

        let _ = &self
            .web_driver
            .set_textarea_value_by_prop_value_async(
                HtmlPropertyNames::NAME,
                InvoicePageNameValues::SERVICE_DESCRIPTION,
                request_model.get_description(),
            )
            .await?;

        let _ = &self
            .web_driver
            .set_input_value_by_prop_value_async(
                HtmlPropertyNames::NAME,
                InvoicePageNameValues::SERVICE_VALUE,
                request_model.get_value(),
            )
            .await?;

        let _ = &self
            .web_driver
            .find_by_prop_value_async(
                HtmlTagNames::SELECT,
                HtmlPropertyNames::NAME,
                InvoicePageNameValues::REGIME_IVA,
            )
            .await?
            // TODO: Add support for different IVA regimes ("regimes de IVA") - by using .reference_data().
            .select_option_by_prop_value_async(
                "label",
                "Regras de localização - art.º 6.º [regras especificas]",
            )
            .await?;

        let _ = &self
            .web_driver
            .find_by_prop_value_async(
                HtmlTagNames::SELECT,
                HtmlPropertyNames::NAME,
                InvoicePageNameValues::REGIME_INCIDENCIA_IRS,
            )
            .await?
            // TODO: Add support for different IVA regimes ("regimes de incidência IRS") - by using .reference_data().
            .select_option_by_prop_value_async(
                "label",
                "Sem retenção - Não residente sem estabelecimento",
            )
            .await?;

        #[cfg(not(feature = "dev-mode"))]
        {
            let _ = &self
                .web_driver
                .find(By::ClassName("fixed-header"))
                .await?
                .find(By::Tag("button"))
                .await?
                .click()
                .await?;

            let _ = &self
                .web_driver
                .find(By::Id("emitirModal"))
                .await?
                .find(By::ClassName("btn-success"))
                .await?
                .click()
                .await?;
        }

        Ok(())
    }
}
