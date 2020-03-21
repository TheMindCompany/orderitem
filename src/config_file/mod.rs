pub mod model;
pub mod handler;

use crate::toolbelt::file_handler::ReadFile;

use std::env;
use std::path::Path;
use dirs;

pub use model::{
    ConfigurationAPI
};
pub use model::host_settings::{
    HostSettings,
};
pub use handler::{
    ConfigurationHandler
};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigurationControl {
    pub config: ConfigurationAPI,
}

impl ConfigurationControl {

    pub fn new() -> ConfigurationControl {
        Default::default()
    }

    pub fn load(mut self) -> ConfigurationControl {
        let config_file = self.load_config("config.yaml");

        self.config = ConfigurationHandler::load_manager_config(&config_file);
        self.set_sql_env();

        self
    }

    pub fn load_config(&mut self, config: &str) -> String {
        let reader = ReadFile::new();
        let mut home = match dirs::home_dir() {
            Some(path) => path,
            None => Path::new(&std::env::var("ORDERITEM_CONFIG_BASE").expect("~/")).to_path_buf(),
        };
        home.push(".orderitem");
        home.push(config);
        reader.load(
            home
        )
    }

    pub fn set_sql_env(&self) {
        self.set_sql_host_env();
        self.set_sql_port_env();
        self.set_sql_username_env();
        self.set_sql_password_env();
    }

    pub fn set_sql_port_env(&self) {
        let host = self.get_credentials();

        match env::var("ORDERITEM_SQL_PORT") {
            Ok(env_val) => {
                if env_val.is_empty() {
                    env::set_var(
                        "ORDERITEM_SQL_PORT",
                        host.port.unwrap(),
                    );
                }
            },
            Err(_) => {
                env::set_var(
                    "ORDERITEM_SQL_PORT",
                    host.port.unwrap(),
                );
            },
        }

    }

    pub fn set_sql_host_env(&self) {
        let host = self.get_credentials();
        match env::var("ORDERITEM_SQL_HOST") {
            Ok(env_val) => {
                if env_val.is_empty() {
                    env::set_var(
                        "ORDERITEM_SQL_HOST",
                        host.host.unwrap(),
                    );
                }
            },
            Err(_) => {
                env::set_var(
                    "ORDERITEM_SQL_HOST",
                    host.host.unwrap(),
                );
            },
        }
    }

    pub fn set_sql_username_env(&self) {
        let host = self.get_credentials();
        match env::var("ORDERITEM_SQL_USERNAME") {
            Ok(env_val) => {
                if env_val.is_empty() {
                    env::set_var(
                        "ORDERITEM_SQL_USERNAME",
                        host.username.unwrap(),
                    );
                }
            },
            Err(_) => {
                env::set_var(
                    "ORDERITEM_SQL_USERNAME",
                    host.username.unwrap(),
                );
            },
        }
    }

    pub fn set_sql_password_env(&self) {
        let host = self.get_credentials();
        match env::var("ORDERITEM_SQL_PASSWORD") {
            Ok(env_val) => {
                if env_val.is_empty() {
                    env::set_var(
                        "ORDERITEM_SQL_PASSWORD",
                        host.password.unwrap(),
                    );
                }
            },
            Err(_) => {
                env::set_var(
                    "ORDERITEM_SQL_PASSWORD",
                    host.password.unwrap(),
                );
            },
        }
    }

    pub fn get_credentials(&self) -> HostSettings {
        self.clone().config.specs.host
    }

}
