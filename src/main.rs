//! # Cell Sim
//!
//! 'cell_sim' is a terminal program used to simulate 1d cell automata.
//! Currently only implemented with Wolfram's Rule 30.
//!

use std::env;
use std::process;

use cell_sim::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem: {}", err);
        process::exit(1);
    });

    if let Err(e) = cell_sim::run(config) {
        eprintln!("Problem: {}", e);
        process::exit(1);
    };
}
