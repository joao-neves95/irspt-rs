pub struct IrsPtUrls {}

impl IrsPtUrls {
    pub const LOGIN_PAGE: &'static str =
        "https://www.acesso.gov.pt/v2/loginForm?partID=PFAP&path=/geral/home";

    pub const ISSUE_INVOICE_PAGE: &'static str =
        "https://irs.portaldasfinancas.gov.pt/recibos/portal/emitir/emitirfatura";
}

pub struct HtmlTagNames {}

impl HtmlTagNames {
    pub const SELECT: &'static str = "select";

    pub const INPUT: &'static str = "select";

    pub const TEXTAREA: &'static str = "textarea";

    pub const OPTION: &'static str = "option";
}

pub struct HtmlPropertyNames {}

impl HtmlPropertyNames {
    pub const NAME: &'static str = "name";

    pub const VALUE: &'static str = "value";
}

pub struct InvoicePageNameValues {}

impl InvoicePageNameValues {
    pub const SERVICE_DATE: &'static str = "dataPrestacao";

    pub const INVOICE_TYPE: &'static str = "tipoRecibo";

    pub const CLIENT_COUNTRY: &'static str = "pais";

    pub const VAT_NATIONAL_CLIENT: &'static str = "nifAdquirente";

    pub const VAT_INTERNATIONAL_CLIENT: &'static str = "nifEstrangeiro";

    pub const CLIENT_NAME: &'static str = "nomeAdquirente";

    pub const CLIENT_ADDRESS: &'static str = "moradaAdquirente";

    pub const SERVICE_DESCRIPTION: &'static str = "servicoPrestado";

    pub const SERVICE_VALUE: &'static str = "valorBase";

    pub const REGIME_IVA: &'static str = "regimeIva";

    pub const REGIME_INCIDENCIA_IRS: &'static str = "regimeIncidenciaIrs";
}
