use structopt::StructOpt;
use super::completions::Completions;

#[derive(StructOpt, Debug, Clone)]
pub enum Configurations {
    /// Set configuration file value to something new.
    #[structopt(name = "set")]
    AppConfiguration(AppConfiguration),

    /// Completion scripts for various shells.
    #[structopt(name = "completions")]
    Completions(Completions),
}

#[derive(StructOpt, Debug, Default, Clone)]
pub struct AppConfiguration {
    /// Field being targeted for change.
    pub field: Option<String>,

    /// Target field new value.
    pub value: Option<String>,
}
