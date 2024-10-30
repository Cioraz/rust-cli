use clap::{Args, Parser, Subcommand};
use colored::{Color, Colorize};

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
    #[arg(short = 'C', long, default_value = "bright_white")]
    color: Option<String>,
}

fn main() {
    let args = Arg::parse();
    match args.cmd {
        Commands::Echo(echo_args) => {
            let count = echo_args.count.unwrap_or(1);
            let color_str = echo_args
                .color
                .unwrap_or(String::from("bright_red"))
                .to_lowercase();

            // Map string color names to Color variants
            let color = match color_str.as_str() {
                "red" => Color::BrightRed,
                "green" => Color::BrightGreen,
                "blue" => Color::BrightBlue,
                "yellow" => Color::BrightYellow,
                "magenta" => Color::BrightMagenta,
                "cyan" => Color::BrightCyan,
                "white" => Color::BrightWhite,
                "bright_red" => Color::BrightRed,
                "bright_green" => Color::BrightGreen,
                "bright_blue" => Color::BrightBlue,
                "bright_yellow" => Color::BrightYellow,
                "bright_magenta" => Color::BrightMagenta,
                "bright_cyan" => Color::BrightCyan,
                "bright_white" => Color::BrightWhite,
                _ => {
                    eprintln!("Invalid color specified. Defaulting to bright red.");
                    Color::BrightRed // Default to bright red for unrecognized colors
                }
            };
            for _ in 0..count {
                println!("{}", echo_args.message.color(color));
            }
        }
    }
}
