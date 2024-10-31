use clap::Args;
use colored::{Color, Colorize};

const DEFAULT_COLOR: &str = "bright_white";

#[derive(Args, Debug, Clone)]
pub struct EchoArgs {
    #[arg(short, long)]
    message: String,
    #[arg(short, long)]
    count: Option<u32>,
    #[arg(short = 'C', long, default_value = DEFAULT_COLOR)]
    color: Option<String>,
}

pub fn execute(echo_args: EchoArgs) {
    let count = echo_args.count.unwrap_or(1);
    let color_str = echo_args
        .color
        .unwrap_or(DEFAULT_COLOR.to_string())
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
            eprintln!("Invalid color specified. Defaulting to bright white.");
            Color::BrightWhite // Default to bright red for unrecognized colors
        }
    };
    for _ in 0..count {
        println!("{}", echo_args.message.color(color));
    }
}
