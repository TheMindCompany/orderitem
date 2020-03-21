#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct HostSettings {
    pub host: Option<String>,
    pub port: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl HostSettings {

    pub fn new() -> HostSettings {
        HostSettings {
            host: Some("localhost".to_string()),
            port: Some("3306".to_string()),
            username: None,
            password: None,
        }
    }

    pub fn from_values(host: Option<String>, port: Option<String>, username: Option<String>, password: Option<String>) -> HostSettings {
        let host = if host.is_none() {host} else {Some("localhost".to_string())};
        let port = if port.is_none() {port} else {Some("3306".to_string())};
        HostSettings {
            host,
            port,
            username,
            password,
        }
    }

}
