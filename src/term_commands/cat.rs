use clap::Args;
use colored::Colorize;
use std::{fs::File, io::BufRead, io::BufReader};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Color, Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;

#[derive(Args, Debug, Clone)]
pub struct CatArgs {
    #[arg(index = 1)]
    filename: Option<String>,
    #[arg(short, long)]
    lineflag: bool,
}

pub fn execute(cat_args: CatArgs) {
    let file_name = cat_args.filename.unwrap_or_else(|| {
        eprintln!("ERROR: No filename provided!");
        std::process::exit(1);
    });

    let file = File::open(&file_name).unwrap_or_else(|error| {
        eprintln!("ERROR: Error opening file {}, {}", file_name, error);
        std::process::exit(1);
    });

    let reader = BufReader::new(file);

    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];

    let extension = file_name.split('.').last().unwrap_or("");
    let syntax = ps
        .find_syntax_by_extension(extension)
        .unwrap_or_else(|| ps.find_syntax_plain_text());

    let mut highlighter = HighlightLines::new(syntax, theme);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap_or_else(|error| {
            eprintln!("ERROR: Unable to read line {}", error);
            String::new()
        });
        let ranges: Vec<(Style, &str)> = highlighter.highlight_line(&line, &ps).unwrap();

        let ranges_no_bg: Vec<(Style, &str)> = ranges
            .into_iter()
            .map(|(mut style, text)| {
                style.background = Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0,
                };
                (style, text)
            })
            .collect();

        let colored_line = as_24_bit_terminal_escaped(&ranges_no_bg[..], false);
        if cat_args.lineflag {
            println!(
                "{}: {}",
                format!("{:4} |", index + 1).bright_blue(),
                colored_line
            );
        } else {
            println!("{}", colored_line);
        }
    }
}
