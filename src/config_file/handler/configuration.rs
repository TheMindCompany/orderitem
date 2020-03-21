use crate::config_file::model::{ConfigurationAPI};
use crate::config_file::handler::GenerateHandler;

use std::env;
use serde_yaml;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigurationHandler {}

impl ConfigurationHandler {

    pub fn load_manager_config(yaml_file: &str) -> ConfigurationAPI {
        let config: ConfigurationAPI = match serde_yaml::from_str(&yaml_file) {
            Ok(x) => x,
            Err(_) => {
                eprintln!("No configuration found. Running generator:\n");
                match env::var("ORDERITEM_SQL_HOST") {
                    Ok(host_val) => {
                        if host_val.is_empty() {
                            GenerateHandler::new().run_config_prompt();
                            std::process::exit(1);
                        } else {
                            ConfigurationAPI::new()
                        }
                    },
                    _ => {
                        GenerateHandler::new().run_config_prompt();
                        std::process::exit(1);
                    },
                }
            },
        };

        config
    }

}
