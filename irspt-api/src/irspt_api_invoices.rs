use anyhow::Result;

use crate::extensions::WebDriverExtensions;
use crate::extensions::WebElementExtensions;
use crate::models::IssueInvoiceRequest;
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
            .set_input_value_by_prop_value_async(
                "name",
                "dataPrestacao",
                request_model.date.as_str(),
            )
            .await?;

        let _ = &self
            .irspt_api
            .web_driver
            .find_elem_by_prop_value_async("select", "name", "tipoRecibo")
            .await?
            .select_option_prop_by_prop_value_async("label", "Fatura-Recibo")
            .await?;

        Ok(())
    }
}
