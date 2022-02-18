#![allow(unused)]

use rlisp::exp::{tokenize, parse};
use rlisp::env::default_env;
use rlisp::eval::eval;

use rlisp::utils::reader::{from_reader};

use std::fs::File;
use std::io::BufReader;

use clap::Parser;
use log::{info, debug, error, LevelFilter};
use simple_logger::SimpleLogger;

#[derive(Parser)]
pub struct CLI {

    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,

    /// Sets the logging level.
    #[clap(
        long="--log-level", 
        default_value="none", 
        possible_values=["none","info","debug"]
    )]
    log_level: String,
}

fn parse_log_level(log_level: &str) -> Option<LevelFilter> {
    match log_level {
        "info" => Some(LevelFilter::Info),
        "debug" => Some(LevelFilter::Debug),
        _ => None
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = CLI::parse();

    let parsed_log_level = parse_log_level(&args.log_level);

    if let Some(level) = parsed_log_level {
        SimpleLogger::new()
            .with_level(level)
            .init()
            .unwrap();
    }

    let mut f = BufReader::new(File::open(&args.path).expect("open failed"));

    let terms = from_reader(f).unwrap();

    let mut env = default_env();

    for term in terms {
        let tokens = tokenize(term);
        let (exp, _) = parse(&tokens[..]).unwrap();
        let value = eval(exp.clone(), &mut env).unwrap();
    
        debug!("{} -> {}", exp, value);
        println!("{}", value);
    }

    Ok(())
}