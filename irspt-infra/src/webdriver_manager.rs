use std::process::{Command, Stdio};

use anyhow::{anyhow, Ok, Result};
use sysinfo::{ProcessRefreshKind, RefreshKind, System, SystemExt};

use irspt_contracts::{
    enums::{InstanceState, WebdriverType},
    traits::TWebdriverManager,
};

pub struct WebdriverManager {
    webdriver_type: WebdriverType,
    sys_info: System,
}

impl TWebdriverManager for WebdriverManager {
    fn new(webdriver_type: WebdriverType) -> Self {
        WebdriverManager {
            webdriver_type,
            sys_info: System::new_with_specifics(
                RefreshKind::new().with_processes(ProcessRefreshKind::new()),
            ),
        }
    }

    fn start_instance_if_needed(&mut self) -> Result<InstanceState> {
        match self.webdriver_type {
            WebdriverType::Gecko => {
                self.sys_info
                    .refresh_processes_specifics(ProcessRefreshKind::new());

                let webdriver_type_str = &self.webdriver_type.to_string();
                let run_driver_error_message = format!(
                    "Error while trying to run the webdriver '{}'.\nCheck if you have a supported driver installed.\nOriginal Error: ",
                    webdriver_type_str
                );

                let proc = self.sys_info.processes_by_name(&webdriver_type_str).next();

                match proc {
                    Some(_) => Ok(InstanceState::Running),
                    None => match Command::new(format!("{}.exe", webdriver_type_str))
                        .stdout(if cfg!(feature = "child-stdout-off") {
                            Stdio::null()
                        } else {
                            Stdio::inherit()
                        })
                        .spawn()
                    {
                        Err(e) => Err(anyhow!(format!("{}{}", run_driver_error_message, e))),
                        anyhow::Result::Ok(child) => Ok(InstanceState::Started(child)),
                    },
                }
            }
            _ => panic!("Unknown webdriver type."),
        }
    }
}

#[cfg(test)]
mod tests {
    use irspt_contracts::{
        enums::{InstanceState, WebdriverType},
        traits::TWebdriverManager,
    };

    use super::WebdriverManager;

    #[test]
    fn is_geckodriver_running_passes() {
        let mut webdriver_client = WebdriverManager::new(WebdriverType::Gecko);
        let final_state = webdriver_client.start_instance_if_needed();

        match final_state {
            Err(e) => panic!("{}", e),

            Ok(state) => match state {
                InstanceState::Started(mut child_command) => {
                    assert!(child_command.id() > 0);
                    let _ = child_command.kill();
                }

                InstanceState::Running => (), // Nothing to do.
                _ => panic!("The final child process state is invalid."),
            },
        };
    }
}
