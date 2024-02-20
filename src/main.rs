use std::env;
use clap::Parser;

mod cli;
mod config_manager;
mod secureshell;


#[derive(Parser)]
struct Cli {
  #[arg(short, long)]
  ssh: Option<String>,
  #[arg(short, long)]
  password: Option<String>,
  #[arg(short, long)]
  add_config: Option<String>,
  #[arg(short, long)]
  remove_config: Option<String>
}

fn main() {
  println!("RUN: sshb");

  let sshb_config = env::var("SSHB_CONFIG")
    .expect("SSHB_CONFIG UNDEFINED");

  let matches = cli::builder().get_matches();
  let config_manager = config_manager::ConfigManager { ssh_config: sshb_config };

  let input_error = "Error: Failed to read input";

  if let Some(host) = matches.get_one::<String>("ssh") {
    println!("CLI COMMAND: ssh {}", host);
    if !config_manager.existing_host(host).unwrap() {
      let message = &format!("Add new host {}?", host.as_str());
      let response = cli::prompt(message, input_error).unwrap();

      if cli::parse_yn(response.as_str()) {
        cli::add_config(&config_manager, host.to_string());
      }
    }
    secureshell::ssh(host, None, None);
  }
  if let Some(_) = matches.get_one::<bool>("list") {
    let hostnames = config_manager.get_hostnames().expect("failed");

    for host in hostnames {
      print!("{}\n", host);
    }
  }
  if let Some(add_config) = matches.get_one::<String>("add_config") {
    cli::add_config(&config_manager, add_config.to_string());
  }
  if let Some(host) = matches.get_one::<String>("remove_config") {
    let _ = config_manager.remove_config(host)
      .expect(format!("Error: Failed to remove config {host}").as_str());
  }
}
