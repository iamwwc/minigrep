use std::{env, fs, process};
use std::error::Error;
use minigrep::Config;
mod utils;
use utils::print;
// use crate::utils::print;
fn main () {
    print();
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error:{}",e);
        process::exit(1);
    }
}