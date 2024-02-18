#![allow(dead_code)]
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};


pub fn add_config(host: &str, hostname: &str, user: &str) -> Result<(), Box<dyn std::error::Error>> {
  println!("fn add_config");
  let mut file = OpenOptions::new().append(true).open("/Users/josephfarrell/.ssh/config")?;

  file.write_all(("Host ".to_owned() + host +
    "\n	HostName " + hostname +
    "\n	User " + user + "\n\n").as_bytes())
    .expect("Unable to write data");

  Ok(())
}

pub fn find_hostname(hostname: &str) -> Result<bool, Box<dyn std::error::Error>> {
  println!("fn: find_hostname");
  let file = File::open("/Users/josephfarrell/.ssh/config")?;
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

pub fn remove_config(host: &str) -> Result<(), Box<dyn std::error::Error>> {
  println!("fn: remove_config");
  let config_file = File::open("/Users/josephfarrell/.ssh/config")?;
  let mut temp_file = File::create("/Users/josephfarrell/.ssh/config.tmp")?;

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

  Ok(())
}
