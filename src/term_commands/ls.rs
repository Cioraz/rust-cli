use clap::Args;
use std::fs;

#[derive(Args, Debug, Clone)]
pub struct LsArgs {
    #[arg(short = 'l', long)]
    list: bool,
    #[arg(short, long)]
    all: bool,
    #[arg(short = 'L', long = "la")]
    la: bool,
    #[arg(index = 1)]
    path: Option<String>,
}

pub fn execute(ls_args: LsArgs) {
    let path = ls_args.path.unwrap_or_else(|| String::from("."));
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let final_entry = entry.path();
                        let final_val = final_entry
                            .strip_prefix("./")
                            .unwrap_or(&final_entry)
                            .display();
                        println!("{} ", final_val);
                    }
                    Err(e) => println!("ERROR: Something went wrong {e}"),
                }
            }
        }
        Err(e) => eprintln!("ERROR: Something went wrong {}", e),
    }
}
