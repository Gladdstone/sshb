use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(allow_external_subcommands = true)]
pub struct Args {
    #[arg(long, short='s')]
    pub ssh: Option<String>,

    #[arg(long, short='n', requires("ssh"))]
    pub hostname: Option<String>,

    #[arg(long, short='l')]
    pub list: bool,

    #[arg(long, short='a', action = ArgAction::Append)]
    pub add_config: Vec<String>,

    #[arg(long, short='r', action = ArgAction::Append)]
    pub remove_config: Vec<String>,

    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(hide = true, external_subcommand)]
    Other(Vec<String>),
}

