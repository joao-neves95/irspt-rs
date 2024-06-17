use crate::enums::{InstanceState, WebdriverType};

pub trait TWebdriverManager {
    fn new(webdriver_type: WebdriverType) -> Self;

    fn start_instance_if_needed(&mut self, is_dev_mode: bool) -> anyhow::Result<InstanceState>;
}
