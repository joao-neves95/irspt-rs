use crate::extensions::ElementProp;
use crate::extensions::WebDriverExtensions;
use crate::extensions::WebElementExtensions;
use crate::IrsptApi;
use irspt_contracts::models::IssueInvoiceRequest;
use irspt_contracts::traits::TIrsptApiInvoices;

use std::thread;
use std::time;

use anyhow::Result;
use async_trait::async_trait;
use thirtyfour::By;

#[async_trait]
impl TIrsptApiInvoices for IrsptApi {
    // TODO: Separate this into a Builder pattern.
    async fn issue_invoice_async(&self, request_model: &IssueInvoiceRequest) -> Result<()> {
        let is_portuguese_client = request_model.get_client_country() == "PORTUGAL";

        // TODO: Un-hardcode url.
        self
            .web_driver
            .goto(format!(
                "https://irs.portaldasfinancas.gov.pt/recibos/portal/emitirfatura#?modoConsulta=Prestador&nifPrestadorServicos={}&isAutoSearchOn=on",
                request_model.get_nif()
            ))
            .await?;

        thread::sleep(time::Duration::from_secs(1));

        let _ = &self
            .web_driver
            .set_input_value_by_prop_value_async("name", "dataPrestacao", request_model.get_date())
            .await?;

        let _ = &self
            .web_driver
            .find_by_prop_value_async("select", "name", "tipoRecibo")
            .await?
            .select_option_by_prop_value_async("label", "Fatura-Recibo")
            .await?;

        thread::sleep(time::Duration::from_millis(500));

        let _ = &self
            .web_driver
            .find_by_prop_value_async("select", "name", "pais")
            .await?
            .select_option_by_prop_value_containing_async(
                "label",
                request_model.get_client_country(),
            )
            .await?;

        let _ = &self
            .web_driver
            .set_input_value_by_prop_value_async(
                "name",
                if is_portuguese_client {
                    "nifAdquirente"
                } else {
                    "nifEstrangeiro"
                },
                request_model.get_client_nif(),
            )
            .await?;

        let _ = &self
            .web_driver
            .set_input_value_by_prop_value_async(
                "name",
                "nomeAdquirente",
                request_model.get_client_name(),
            )
            .await?;

        if is_portuguese_client {
            let _ = &self
                .web_driver
                .set_input_value_by_prop_value_async(
                    "name",
                    "moradaAdquirente",
                    request_model.get_client_address(),
                )
                .await;
        }

        let _ = &self
            .web_driver
            .find_by_props_value_async(
                "input",
                &[
                    &ElementProp {
                        prop_name: "name",
                        prop_value: "titulo",
                    },
                    // TODO: Add support for different titles of payment ("títulos de pagamento") - by using .reference_data().
                    &ElementProp {
                        prop_name: "value",
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
                "name",
                "servicoPrestado",
                request_model.get_description(),
            )
            .await?;

        let _ = &self
            .web_driver
            .set_input_value_by_prop_value_async("name", "valorBase", request_model.get_value())
            .await?;

        let _ = &self
            .web_driver
            .find_by_prop_value_async("select", "name", "regimeIva")
            .await?
            // TODO: Add support for different IVA regimes ("regimes de IVA") - by using .reference_data().
            .select_option_by_prop_value_async(
                "label",
                "Regras de localização - art.º 6.º [regras especificas]",
            )
            .await?;

        let _ = &self
            .web_driver
            .find_by_prop_value_async("select", "name", "regimeIncidenciaIrs")
            .await?
            // TODO: Add support for different IVA regimes ("regimes de incidência IRS") - by using .reference_data().
            .select_option_by_prop_value_async(
                "label",
                "Sem retenção - Não residente sem estabelecimento",
            )
            .await?;

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

        Ok(())
    }
}
