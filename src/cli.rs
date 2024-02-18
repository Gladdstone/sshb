use clap::{command, Arg, ArgAction, Command};


pub fn builder() -> Command {
  command!()
    .args([
      Arg::new("ssh")
        .short('s')
        .long("ssh")
        .action(ArgAction::Set)
        .help("help text"),
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