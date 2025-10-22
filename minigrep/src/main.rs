use minigrep::{search, search_case_insensitive};
use std::{env, error::Error, fs, process};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!(
        "In file {}, case: {}\n------------------------------------",
        config.file_path, config.ignore_case
    );
    if let Err(e) = Config::run(&config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub ignore_case: bool,
}

impl<'a> Config<'a> {
    fn build(args: &'_ [String]) -> Result<Config<'_>, &'_ str> {
        if args.len() < 3 {
            return Err(
                "not enough arguments, \nUses -> minigrep 'pattern' 'file_path' 'true/false(optional, for case-insensitive)'",
            );
        }

        let ignore_case_ = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query: &args[1],
            file_path: &args[2],
            ignore_case: if args.len() == 4 {
                args[3].parse::<bool>().unwrap()
            } else {
                ignore_case_
            },
        })
    }

    fn run(cfg: &Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(cfg.file_path)
            .expect("Error reading the file {file_path}, make sure file path is correct.");
        let results = if cfg.ignore_case {
            search_case_insensitive(cfg.query, &contents)
        } else {
            search(cfg.query, &contents)
        };
        for line in results {
            println!("{line}");
        }
        // println!("With text:\n{contents}");
        Ok(())
    }
}
