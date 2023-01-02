use strum::Display;

#[derive(Debug, Display)]
pub enum WebdriverType {
    #[strum(serialize = "geckodriver")]
    Gecko,
}
