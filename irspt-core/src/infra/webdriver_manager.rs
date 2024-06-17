use crate::{
    enums::{InstanceState, WebdriverType},
    traits::TWebdriverManager,
};

use std::process::{Command, Stdio};

use anyhow::{anyhow, Ok, Result};
use sysinfo::{ProcessRefreshKind, RefreshKind, System, SystemExt};

pub struct WebdriverManager {
    webdriver_type: WebdriverType,
}

impl TWebdriverManager for WebdriverManager {
    fn new(webdriver_type: WebdriverType) -> Self {
        WebdriverManager { webdriver_type }
    }

    fn start_instance_if_needed(&mut self, is_dev_mode: bool) -> Result<InstanceState> {
        let mut sys_info = System::new_with_specifics(
            RefreshKind::new().with_processes(ProcessRefreshKind::new()),
        );
        sys_info.refresh_processes_specifics(ProcessRefreshKind::new());

        let webdriver_type_str = &self.webdriver_type.to_string();
        let run_driver_error_message = format!(
            "Error while trying to run the webdriver '{}'.\nCheck if you have a supported driver installed.\nOriginal Error: ",
            webdriver_type_str
        );

        let proc = sys_info.processes_by_name(&webdriver_type_str).next();

        match proc {
            Some(_) => Ok(InstanceState::Running),

            None => match Command::new(webdriver_type_str)
                .stdout(if !is_dev_mode {
                    Stdio::null()
                } else {
                    Stdio::inherit()
                })
                .spawn()
            {
                Err(e) => Err(anyhow!(format!("{}{:#?}", run_driver_error_message, e))),

                anyhow::Result::Ok(child) => Ok(InstanceState::Started(child)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WebdriverManager;
    use crate::{
        enums::{InstanceState, WebdriverType},
        traits::TWebdriverManager,
    };

    #[test]
    fn is_geckodriver_running_passes() {
        let mut webdriver_client = WebdriverManager::new(WebdriverType::Gecko);
        let final_state_result = webdriver_client.start_instance_if_needed(false);

        // Cleanup.
        match final_state_result {
            Err(e) => panic!("{}", e),

            Ok(state) => match state {
                // Do nothing if it was already running on the client machine.
                InstanceState::Running => (),

                InstanceState::Started(mut child_command) => {
                    assert!(child_command.id() > 0);
                    let _ = child_command.kill();
                }

                _ => panic!("The final child process state is invalid."),
            },
        };
    }
}
