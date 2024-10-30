use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author,version,about,long_about=None)]
struct Arg {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Echo(EchoArgs),
}

#[derive(Args, Debug, Clone)]
struct EchoArgs {
    #[arg(short, long)]
    message: String,
    #[arg(short, long)]
    count: Option<u32>,
}

fn main() {
    let args = Arg::parse();
    match args.cmd {
        Commands::Echo(echo_args) => {
            let count = echo_args.count.unwrap_or(1);
            for _ in 0..count {
                println!("{}", echo_args.message);
            }
        }
    }
}
