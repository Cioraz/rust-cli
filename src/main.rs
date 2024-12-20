mod term_commands;

use clap::{Parser, Subcommand};
use term_commands::cat::CatArgs;
use term_commands::echo::EchoArgs;
use term_commands::ls::LsArgs;

#[derive(Parser, Debug)]
#[command(author,version,about,long_about=None)]
struct Arg {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Echo(EchoArgs),
    Cat(CatArgs),
    Ls(LsArgs),
}

fn main() {
    let args = Arg::parse();
    match args.cmd {
        Commands::Echo(echo_args) => {
            term_commands::echo::execute(echo_args);
        }
        Commands::Cat(cat_args) => {
            term_commands::cat::execute(cat_args);
        }
        Commands::Ls(ls_args) => {
            term_commands::ls::execute(ls_args);
        }
    }
}
