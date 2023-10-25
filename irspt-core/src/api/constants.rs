pub struct IrsPtUrls {}

impl IrsPtUrls {
    pub const LOGIN_PAGE: &str = "https://www.acesso.gov.pt/v2/loginForm?partID=PFAP&path=/geral/home";

    pub const ISSUE_INVOICE_PAGE: &str =
        "https://irs.portaldasfinancas.gov.pt/recibos/portal/emitir/emitirfatura";
}

pub struct InvoicePageNameValues {}

impl InvoicePageNameValues {
    pub const SERVICE_DATE: &str = "dataPrestacao";

    pub const INVOICE_TYPE: &str = "tipoRecibo";

    pub const CLIENT_COUNTRY: &str = "pais";

    pub const VAT_NATIONAL_CLIENT: &str = "nifAdquirente";

    pub const VAT_INTERNATIONAL_CLIENT: &str = "nifEstrangeiro";

    pub const CLIENT_NAME: &str = "nomeAdquirente";

    pub const CLIENT_ADDRESS: &str = "moradaAdquirente";

    pub const SERVICE_DESCRIPTION: &str = "servicoPrestado";

    pub const SERVICE_VALUE: &str = "valorBase";
}
