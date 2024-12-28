use std::env;

use client::cli;
use client::args::{Commands};
use ssh::config_manager;

mod client;
mod ssh;
mod file_utils;
mod secureshell;


fn main() {
  dbg!("RUN: sshb");

  // fall back to a default path if the variable isn't set
  let sshb_config = env::var("SSHB_CONFIG")
    .unwrap_or_else(|_| {
        return file_utils::get_default_config_path();
    });

  let config_manager = config_manager::ConfigManager {
      ssh_config: sshb_config
  };

  let cli = cli::Cli::new(std::env::args().len(), &config_manager);
  dbg!(cli.arg_len);

  match &cli.args.command {
    Some(Commands::Other(args)) => {
      cli.ssh(&args[0]);
    },
    None => {
      if cli.args.list {
        cli.list()
      }
      if cli.args.add_config.len() > 0 {
        let host = &cli.args.add_config[0];
        cli.add_config(host);
      }
      if cli.args.remove_config.len() > 0 {
        let host = &cli.args.remove_config[0];
        cli.remove_config(host);
      }
      if cli.args.ssh.is_some() {
        cli.select_ssh();
      }
    }
  }
}

