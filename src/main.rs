#[allow(unused_imports)]
#[macro_use]
extern crate slog;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;
extern crate env_logger;
extern crate reqwest;
extern crate run_script;
extern crate sys_info;
extern crate regex;
extern crate dialoguer;
extern crate console;
extern crate dirs;
extern crate mysql_async;
extern crate actix_web;
extern crate actix_rt;
extern crate tokio;
extern crate structopt;
extern crate uuid;
extern crate url;

mod cli;
mod daemon;
mod command_control;
mod orders;
mod toolbelt;
mod prompt_user;
mod config_file;

use cli::Cli;
use daemon::Daemeon;
use structopt::StructOpt;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // This is the collection of settings sent from the request.
    let cli_options = command_control::CmdCtl::from_args();

    config_file::ConfigurationControl::new().load();

    if !cli_options.daemon {
        Cli::run_as_cli().await;
        Ok(())
    } else {
        Daemeon::run_as_daemeon().await
    }
}
