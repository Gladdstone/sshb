use std::io::{self, Write};
use clap::{Parser, value_parser};

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

  // let matches = cli::builder().get_matches();
  // let values = Value::from_matches(&matches);

  let args = Cli::parse();

  if let Some(host) = args.ssh.as_deref() {
    println!("CLI COMMAND: ssh {}", host);
    secureshell::ssh(host, None, None);
  }
  if let Some(add_config) = args.add_config.as_deref() {
    let host = add_config;
    let mut hostname = String::new();
    let mut user = String::new();
    let mut password = String::new();

    print!("Enter hostname: ");
    let _ = std::io::stdout().flush();
    io::stdin().read_line(&mut hostname)
      .expect("Error: Failed to read input");

    print!("Enter user: ");
    let _ = std::io::stdout().flush();
    io::stdin().read_line(&mut user)
      .expect("Error: Failed to read input");

    print!("Enter password: ");
    let _ = std::io::stdout().flush();
    io::stdin().read_line(&mut password)
      .expect("Error: Failed to read input");

    config_manager::add_config(host, hostname.as_str(), user.as_str())
      .expect(format!("Error: Failed to add config {host}").as_str());
  }
  if let Some(host) = args.remove_config.as_deref() {
    let _ = config_manager::remove_config(host)
      .expect(format!("Error: Failed to remove config {host}").as_str());
  }
}
