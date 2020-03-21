use crate::config_file::ConfigurationAPI;
use crate::config_file::model::host_settings::{HostSettings};
use crate::toolbelt::file_handler::CreateFile;

use crate::config_file::model::{
    ConfigurationSpec
};
use std::path::Path;

use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Prompt {
    pub name: String,
    pub default: Option<String>,
}

impl Prompt {
    pub fn new() -> Prompt {
        Default::default()
    }

    pub fn set_name(mut self, name: String) -> Prompt {
        self.name = name;
        self
    }

    pub fn default() -> String {
        "None".to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GenerateHandler {
    pub config: ConfigurationAPI,
}

impl GenerateHandler {

    pub fn new() -> GenerateHandler {
        Default::default()
    }

    pub fn run_config_prompt(&mut self) -> GenerateHandler {
        let config = self.get_config();
        let mut config_path = match dirs::home_dir() {
            Some(path) => path,
            None => Path::new(&std::env::var("ORDERITEM_CONFIG_BASE").expect("~/")).to_path_buf(),
        };
        config_path.push(".OrderItem");
        match std::fs::create_dir(&config_path) {
            Ok(_) => {
                println!("Created path ~/.orderitem");
            },
            Err(_) => {
                println!("Path ~/.orderitem already exist.");
            }
        };
        config_path.push("config.yaml");

        CreateFile::new().create(config_path, &serde_yaml::to_string(&config).unwrap());

        self.clone()
    }

    pub fn get_user_credentials(&mut self) -> ConfigurationSpec {
        let host = PromptHandler::process( Prompt::new().set_name("sql_host".to_string()) );
        let port = PromptHandler::process( Prompt::new().set_name("sql_port".to_string()) );
        let username = PromptHandler::process( Prompt::new().set_name("sql_username".to_string()) );
        let password = PromptHandler::process( Prompt::new().set_name("sql_password".to_string()) );

        ConfigurationSpec::from_values(
            HostSettings::from_values(
                Some(host),
                Some(port),
                Some(username),
                Some(password))
        )
    }

    pub fn get_config(&mut self) -> ConfigurationAPI {
        let spec = self.get_user_credentials();
        ConfigurationAPI::from_values(spec)
    }

}

// Move this to shared module when we know the variance between prompt requirments.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PromptHandler {}

impl PromptHandler {
    pub fn process(prompt: Prompt) -> String {
        PromptHandler::ask_input(
            Prompt::default(),
            &prompt.name,
        )
    }


    // Prompt for user defined input.
    pub fn ask_input(
        default_value: String,
        name: &str,
    ) -> String {
        let question = format!("{} should be set?", &name);
        let color_theme = ColorfulTheme::default();
        let mut prompt = Input::with_theme(&color_theme);
        prompt.default(default_value);

        prompt
            .with_prompt(&question.as_str())
            .interact()
            .unwrap()
    }
}
