use crate::orders::*;
use crate::command_control;
use crate::toolbelt::logs::RootLog;
use structopt::StructOpt;

pub struct Cli { }

impl Cli {
    pub async fn run_as_cli() {
        // This is the collection of settings sent from the request.
        let cli_options = command_control::CmdCtl::from_args();

        // Seed table incase it isn't present.  This should be ran safely without worry of overwritting.
        if cli_options.seed {
            OrderCreate::create_table().await.unwrap();
            std::process::exit(0)
        }

        // This should be passed to any underlying modules and follow verbose logic rules for CLI.
        let log_config = RootLog::get_logger(
            cli_options.is_verbose()
        );

        match cli_options.commands {
            Some(command_control::Commands::Configurations(_)) => {
                cli_options.clone().run_command_process();
            },
            None => {
                match OrderItemRunner::run(&cli_options).await {
                    Ok(x) => info!(log_config, "{:#?}", x),
                    Err(err) => debug!(log_config, "{:?}", err),
                }
            }
        }
    }
}
