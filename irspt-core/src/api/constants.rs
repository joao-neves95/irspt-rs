pub struct IrsPtUrls {}

impl IrsPtUrls {
    pub const LOGIN_PAGE: &str =
        "https://www.acesso.gov.pt/v2/loginForm?partID=PFAP&path=/geral/home";

    pub const ISSUE_INVOICE_PAGE: &str =
        "https://irs.portaldasfinancas.gov.pt/recibos/portal/emitir/emitirfatura";
}

pub struct HtmlTagNames {}

impl HtmlTagNames {
    pub const SELECT: &str = "select";

    pub const INPUT: &str = "select";

    pub const TEXTAREA: &str = "textarea";

    pub const OPTION: &str = "option";
}

pub struct HtmlPropertyNames {}

impl HtmlPropertyNames {
    pub const NAME: &str = "name";

    pub const VALUE: &str = "value";
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

    pub const REGIME_IVA: &str = "regimeIva";

    pub const REGIME_INCIDENCIA_IRS: &str = "regimeIncidenciaIrs";
}
