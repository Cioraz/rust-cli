use clap::Args;
use std::{fs::File, io::BufRead, io::BufReader};

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

    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => {
                if cat_args.lineflag {
                    println!("{}: {}", index + 1, content);
                } else {
                    println!("{}", content);
                }
            }
            Err(error) => eprintln!("ERROR: Something went wrong: {}", error),
        }
    }
}
