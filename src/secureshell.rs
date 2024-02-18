use std::process::Command;


pub fn ssh(host: &str, user: Option<&str>, password: Option<&str>) {
  let mut ssh_command = String::new();
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
    let _ = Command::new("ssh")
      .arg("root@24.199.108.15")
      .spawn()
      .expect("Error: failed to spawn SSH process");
  }
}
