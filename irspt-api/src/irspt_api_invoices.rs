use anyhow::Result;
use thirtyfour::By;

use irspt_core::models::IssueInvoiceRequest;

use crate::extensions::ElementProp;
use crate::extensions::WebDriverExtensions;
use crate::extensions::WebElementExtensions;
use crate::IrsptApi;

pub struct IrsptApiInvoices<'a> {
    irspt_api: &'a IrsptApi,
}

impl<'a> IrsptApiInvoices<'a> {
    pub fn new(irspt_api: &'a IrsptApi) -> IrsptApiInvoices {
        IrsptApiInvoices { irspt_api }
    }

    pub fn get_reference_data() -> () {
        todo!()
    }

    // TODO: Separate this into multiple functions and then create a Builder pattern.
    pub async fn issue_invoice(&self, request_model: IssueInvoiceRequest) -> Result<()> {
        // TODO: Un-hardcode url.
        self.irspt_api
            .web_driver
            .goto(format!(
                "https://irs.portaldasfinancas.gov.pt/recibos/portal/emitirfatura#?modoConsulta=Prestador&nifPrestadorServicos={}&isAutoSearchOn=on",
                request_model.nif
            ))
            .await?;

        let _ = &self
            .irspt_api
            .web_driver
            .set_input_value_by_prop_value_async("name", "dataPrestacao", &request_model.date)
            .await?;

        let _ = &self
            .irspt_api
            .web_driver
            .find_elem_by_prop_value_async("select", "name", "tipoRecibo")
            .await?
            .select_option_by_prop_value_async("label", "Fatura-Recibo")
            .await?;

        let _ = &self
            .irspt_api
            .web_driver
            .set_input_value_by_prop_value_async(
                "name",
                "nifAdquirente",
                &request_model.client_nif.to_string(),
            )
            .await?;

        let _ = &self
            .irspt_api
            .web_driver
            .set_input_value_by_prop_value_async(
                "name",
                "nomeAdquirente",
                &request_model.client_name.to_string(),
            )
            .await?;

        let _ = &self
            .irspt_api
            .web_driver
            .set_input_value_by_prop_value_async(
                "name",
                "moradaAdquirente",
                &request_model.client_address.to_string(),
            )
            .await?;

        let _ = &self
            .irspt_api
            .web_driver
            .find_elem_by_props_value_async(
                "input",
                &[
                    &ElementProp {
                        prop_value: "name",
                        prop_name: "titulo",
                    },
                    // TODO: Add support for different titles of payment ("títulos de pagamento") - by using .reference_data().
                    &ElementProp {
                        prop_value: "value",
                        prop_name: "1",
                    },
                ],
            )
            .await?
            .click()
            .await?;

        let _ = &self
            .irspt_api
            .web_driver
            .set_input_value_by_prop_value_async(
                "name",
                "servicoPrestado",
                &request_model.build_description(),
            )
            .await?;

        let _ = &self
            .irspt_api
            .web_driver
            .set_input_value_by_prop_value_async("name", "valorBase", &request_model.value)
            .await?;

        let _ = &self
            .irspt_api
            .web_driver
            .find_elem_by_prop_value_async("select", "name", "regimeIva")
            .await?
            // TODO: Add support for different IVA regimes ("regimes de IVA") - by using .reference_data().
            .select_option_by_prop_value_async(
                "label",
                "Regras de localização - art.º 6.º [regras especificas]",
            )
            .await?;

        let _ = &self
            .irspt_api
            .web_driver
            .find_elem_by_prop_value_async("select", "name", "regimeIncidenciaIrs")
            .await?
            // TODO: Add support for different IVA regimes ("regimes de incidência IRS") - by using .reference_data().
            .select_option_by_prop_value_async(
                "label",
                "Sem retenção - Não residente sem estabelecimento",
            )
            .await?;

        let _ = &self
            .irspt_api
            .web_driver
            .find(By::ClassName("fixed-header"))
            .await?
            .find(By::Tag("button"))
            .await?
            .click()
            .await?;

        Ok(())
    }
}
