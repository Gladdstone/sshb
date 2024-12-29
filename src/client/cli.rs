use std::io::{self, Write};
use clap::{ Parser};
use crate::{secureshell, ssh::config_manager};
use crate::client::args::{Args};

const INPUT_ERROR: &str = "Failed to read input";

// #[derive(Debug)]
/// CLI Object
pub struct Cli<'a> {
    /// Clap arguments
    pub args: Args,
    /// Number of arguments passed by user for current session
    pub arg_len: usize,
    /// ConfigManager object
    pub config_manager: &'a config_manager::ConfigManager,
}

impl<'a> Cli<'a> {

    pub fn new(arg_len: usize, config_manager: &'a config_manager::ConfigManager) -> Self {
        let args = Args::parse();
        Self { args: args, arg_len: arg_len, config_manager: config_manager }
    }

    pub fn add_config(&self, host: &str) {
      let hostname = self.prompt("Enter hostname: ").unwrap();
      let user = self.prompt("Enter user: ").unwrap();
      let password = self.prompt("Enter password: ").unwrap();

      self.config_manager.add_config(&host, &hostname, &user)
        .expect(format!("Error: Failed to add config {host}").as_str());
    }


    pub fn list(&self) {
      let hostnames = self.config_manager.get_hostnames().expect("failed");

      for host in hostnames {
        print!("{}\n", host);
      }
    }

    pub fn parse_yn(&self, value: &str) -> bool {
      let lower_value = value.to_lowercase();

      match lower_value.as_str() {
        "yes" => return true,
        "y" => return true,
        "no" => return false,
        "n" => return false,
        _ => return false
      };
    }

    pub fn prompt(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
      let mut user_input = String::new();


      print!("{}", message);
      let _ = std::io::stdout().flush();
      io::stdin().read_line(&mut user_input)
        .expect(&format!("{}", INPUT_ERROR));

      return Ok(user_input);
    }

    pub fn remove_config(&self, host: &str) {
        self.config_manager.remove_config(host).expect("Error: Failed to remove config");
    }

    /// Default ssh function
    /// If passed hostname does not exist in .ssh/config, will ask to add it
    ///
    /// Arguments:
    ///
    /// * `hostname`: target hostname or ipaddress
    pub fn ssh(&self, hostname: &str) {
        if !self.config_manager.existing_host(hostname).unwrap() {
            let message = &format!("Add new host {}", hostname);
            let response = &self.prompt(message).unwrap();

            if self.parse_yn(response) {
                self.add_config(hostname);
            }

            secureshell::ssh(hostname, None, None);
        }
    }

    /// SSH function if the ssh flag is passed.
    /// Prints and selects from all available destinations in config.
    pub fn select_ssh(&self) {
        let hostnames = &self.config_manager.get_hostnames().expect("failed");

        let mut i = 1;
        for host in hostnames {
            print!("{}: {}\n", i, host);
            i += 1;
        }

        let input = self.prompt("Select: ");
        let input_int = input.unwrap().parse::<i32>().expect("Error: Please enter a valid integer value");
        if input_int < 0 || input_int > hostnames.len().try_into().unwrap() {
            panic!("Error: Please enter a valid integer");
        }
        secureshell::ssh(&hostnames[1], None, None);
    }

}

