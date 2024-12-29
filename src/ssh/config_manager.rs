use std::fs::{File, OpenOptions, rename};
use std::io::{BufRead, BufReader, Error, Write};


#[derive(Debug)]
pub struct ConfigManager {
  pub ssh_config: String
}

impl ConfigManager {

  pub fn add_config(&self, host: &str, hostname: &str, user: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new().append(true).open(format!("{}", self.ssh_config))?;

    file.write_all(("Host ".to_owned() + host +
      "\n	HostName " + hostname +
      "	User " + user + "\n").as_bytes())
      .expect("Unable to write data");

    Ok(())
  }

  pub fn get_hostnames(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(format!("{}", self.ssh_config))
      .expect("Error: unable to open config");
    let reader = BufReader::new(file);

    let mut hostnames: Vec<String> = Vec::new();
    for line in reader.lines() {
      let line = line?;

      if line.chars().count() > 4 {
        if &line[0..4] == "Host" {
          hostnames.push(line[5..].to_string());
        }
      }
    }

    return Ok(hostnames);
  }

  /// Opens the configured ssh config, searches for the passed hostname.
  /// Returns true is hostname is present, false otherwise.
  ///
  /// Arguments:
  ///
  /// * `hostname`: target hostname or ipaddress
  pub fn existing_host(&self, hostname: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let file = File::open(format!("{}", self.ssh_config))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
      let line = line?;

      if line.chars().count() >= 8 {
        if &line[8..] == hostname {
          return Ok(true);
        }
      }
    }

    return Ok(false);
  }

  pub fn remove_config(&self, host: &str) -> Result<(), Error> {
    let config_file = File::open(format!("{}", self.ssh_config))?;

    let temp_config = &format!("{}.tmp", self.ssh_config);
    let mut temp_file = File::create(temp_config)?;

    let reader = BufReader::new(config_file);

    let mut lines = reader.lines();
    while let Some(line) = lines.next() {
      let line_str = line.unwrap();
      if line_str.chars().count() < 5 {
        temp_file.write_all((line_str + "\n").as_bytes()).expect("Error: Unable to write to file");
        continue;
      }
      if &line_str[0..4] == "Host" && &line_str[5..] == host {
        println!("Removed:");
        println!("{}", &line_str);
        while let Some(line) = lines.next() {
          let line_str = line.unwrap();
          if line_str == "" {
            break;
          }
          println!("{}", line_str);
        }
      } else {
        temp_file.write_all((line_str + "\n").as_bytes()).expect("Error: Unable to write to file");
      }
    }

    return rename(temp_config, &self.ssh_config);
  }

}
