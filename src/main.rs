use std::env;

mod cli;
mod config_manager;
mod file_utils;
mod secureshell;

const INPUT_ERROR: &str = "Failed to read input";

fn main() {
  println!("RUN: sshb");

  // this should try to fall back to a default path if the variable isn't set
  let sshb_config = env::var("SSHB_CONFIG")
    .unwrap_or_else(|_| {
        return file_utils::get_default_config_path();
    });

  let matches = cli::builder().get_matches();
  // cli::init_cli(&matches);

  let config_manager = config_manager::ConfigManager { ssh_config: sshb_config };

  // if matches.args_present() {
    // let hostnames = &config_manager.get_hostnames().expect("failed");

    // let mut i = 1;
    // for host in hostnames {
    //   print!("{}: {}\n", i, host);
    //   i += 1;
    // }

    // let input = cli::prompt("Select: ", INPUT_ERROR);

    // let input_int = input.unwrap().parse::<i32>().expect("Error: Please enter a valid integer value");
    // if input_int < 0 || input_int > hostnames.len().try_into().unwrap() {
    //     panic!("Error: Please enter a valid integer value");
    // }
    // secureshell::ssh(hostnames[1].as_str(), None, None);
  // }
  
  if let Some(host) = matches.get_one::<String>("ssh") {
    println!("CLI COMMAND: ssh {}", host);
    if !config_manager.existing_host(host).unwrap() {
      let message = &format!("Add new host {}?", host.as_str());
      let response = cli::prompt(message, INPUT_ERROR).unwrap();

      if cli::parse_yn(response.as_str()) {
        cli::add_config(&config_manager, host.to_string());
      }
    }
    secureshell::ssh(host, None, None);
  }
  if *matches.get_one::<bool>("list").unwrap() {
    cli::list(&config_manager);
  }
  if let Some(add_config) = matches.get_one::<String>("add_config") {
    cli::add_config(&config_manager, add_config.to_string());
  }
  if let Some(host) = matches.get_one::<String>("remove_config") {
    let _ = config_manager.remove_config(host)
      .expect(format!("Error: Failed to remove config {host}").as_str());
  }
}
