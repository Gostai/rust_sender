mod start; 
pub use start::*;
use clap::Parser;
use anyhow::Result;

#[derive(Debug, Parser)]
#[clap(name = "snarkOS", author = "The Aleo Team <hello@aleo.org>", setting = clap::AppSettings::ColoredHelp)]
pub struct CLI {
    /// Specify the verbosity [options: 0, 1, 2, 3]
    #[clap(default_value = "2", short, long)]
    pub verbosity: u8,
    /// Specify a subcommand.
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {    
    #[clap(name = "start")]
    Start(Box<Start>),    
}

impl Command {
    /// Parses the command.
    pub fn parse(self) -> Result<String> {
        match self {            
            Self::Start(command) => command.parse(),           
        }
    }
}
