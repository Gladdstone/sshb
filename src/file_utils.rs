use std::fs;


fn delete_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
  fs::remove_file(path).expect("Error: Unable to delete file");

  return Ok(());
}

fn get_filename(path: &str) {
  let split_filepath = path.split("/").collect();
  return split_filepath[-1];
}

fn get_filepath(path: &str) {
  let split_filepath = path.split("/").collect();
}

fn rename_file(old: &str, new: &str) -> Result<(), Box<dyn std::error::Error>> {
  fs::rename(old, new).expect("Error: Unable to rename file");

  return Ok(());
}
