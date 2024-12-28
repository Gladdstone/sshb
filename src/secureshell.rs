use std::process::Command;


/// execute ssh command
///
/// Arguments:
///
/// * `host`: target hostname or ip address
/// * `user`: user, if specified
/// * `password`: password, if specified
pub fn ssh(host: &str, user: Option<&str>, password: Option<&str>) {
  let ssh_command;
  if let Some(user) = user.as_deref() {
    ssh_command = format!("ssh {user}@{host}");
  } else {
    ssh_command = format!("ssh {host}");
  }

  if let Some(password) = password.as_deref() {
    let _ = Command::new(ssh_command)
      .arg(format!("-p {password}"))
      .spawn()
      .expect("Error: failed to spawn SSH process");
  } else {
    let _ = Command::new(ssh_command)
      .spawn()
      .expect("Error: failed to spawn SSH process");
  }
}
