use std::io::{self, Write};

use clap::{command, Arg, ArgAction, Command};

use crate::config_manager::ConfigManager;


pub fn builder() -> Command {
  command!()
    .args([
      Arg::new("ssh")
        .short('s')
        .long("ssh")
        .action(ArgAction::Set)
        .help("help text"),
      Arg::new("list")
        .short('l')
        .long("list")
        .action(ArgAction::SetTrue),
      Arg::new("add_config")
        .short('a')
        .long("add-config")
        .action(ArgAction::Append),
      Arg::new("remove_config")
        .short('r')
        .long("remove-config")
        .action(ArgAction::Append)
    ])
}

pub fn prompt(message: &str, error_message: &str) -> Result<String, Box<dyn std::error::Error>> {
  let mut user_input = String::new();


  print!("{}", message);
  let _ = std::io::stdout().flush();
  io::stdin().read_line(&mut user_input)
    .expect(&format!("{}", error_message));

  return Ok(user_input);
}

pub fn parse_yn(value: &str) -> bool {
  let lower_value = value.to_lowercase();

  match lower_value.as_str() {
    "yes" => return true,
    "y" => return true,
    "no" => return false,
    "n" => return false,
    _ => return false
  };
}

pub fn add_config(config_manager: &ConfigManager, host: String) {
  let mut hostname = String::new();
  let mut user = String::new();
  let mut password = String::new();

  let input_error = "Error: Failed to read input";

  hostname = prompt("Enter hostname: ", input_error).unwrap();
  user = prompt("Enter user: ", input_error).unwrap();
  password = prompt("Enter password: ", input_error).unwrap();

  config_manager.add_config(&host, hostname.as_str(), user.as_str())
    .expect(format!("Error: Failed to add config {host}").as_str());
}
