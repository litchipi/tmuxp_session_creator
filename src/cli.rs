use structopt::StructOpt;

use crate::errors::Errcode;

pub mod create;
pub mod edit;

use create::TmuxpSessionCreation;
use edit::TmuxpSessionEdition;

macro_rules! cli_commands {
    ($($name:ident => $impl:ident),+) => {
        #[derive(Debug, StructOpt)]
        #[structopt(name = "tmuxph", about = "Manages tmuxp JSON files")]
        pub enum Commands {
            $(
                $name($impl),
            )*
        }

        impl Commands {
            pub fn start(&self) -> Result<(), Errcode>{
                match self {
                    $(
                        Commands::$name(c) => subcmd(c),
                    )*
                }
            }
        }
    };
}

pub trait CliSubCommand {
    fn validate_args(&self) -> Result<(), Errcode>;
    fn execute_command(&self) -> Result<(), Errcode>;
}

cli_commands!(
    Create => TmuxpSessionCreation,
    Edit => TmuxpSessionEdition
);

pub fn subcmd<T: CliSubCommand>(args: &T) -> Result<(), Errcode> {
    args.validate_args()?;
    args.execute_command()
}
