#![allow(unused)]
use std::env;
use std::fs;


fn delete_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
  fs::remove_file(path).expect("Error: Unable to delete file");

  return Ok(());
}

pub fn get_default_config_path<'a>() -> String {
    let home_env = env::var("HOME").unwrap();
    let default_config_path = home_env + "/.ssh/config";    
    return default_config_path;
}

fn get_filename(path: &str) -> &str {
let split_filepath: Vec<&str> = path.split("/").collect();
  return *split_filepath.last().unwrap();
}

fn get_filepath(path: &str) {
  let split_filepath: Vec<&str> = path.split("/").collect();
}

fn rename_file(old: &str, new: &str) -> Result<(), Box<dyn std::error::Error>> {
  fs::rename(old, new).expect("Error: Unable to rename file");

  return Ok(());
}

