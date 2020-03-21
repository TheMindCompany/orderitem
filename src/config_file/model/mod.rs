pub mod host_settings;
pub use host_settings::HostSettings;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigurationAPI {
    kind: String,
    version: String,
    pub specs: ConfigurationSpec,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigurationSpec {
    pub host: HostSettings,
}

impl ConfigurationAPI {

    pub fn new() -> ConfigurationAPI {
        ConfigurationAPI::from_values(
            ConfigurationSpec::from_values(
                HostSettings::new()
            )
        )
    }

    pub fn from_values(spec: ConfigurationSpec) -> ConfigurationAPI {
        ConfigurationAPI {
            kind: String::from("config"),
            version: String::from("alpha/1.0"),
            specs: spec,
        }
    }

}

impl ConfigurationSpec {

    pub fn from_values(host: HostSettings) -> ConfigurationSpec {
        ConfigurationSpec {
            host,
        }
    }

}
