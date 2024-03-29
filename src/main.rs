//! # Cell Sim
//!
//! 'cell_sim' is a terminal program used to simulate 1d cell automata.
//! Currently only implemented with Wolfram's Rule 30.
//!

use std::process;

use cell_sim::Config;
use clap::Parser;

fn main() {
    let config = Config::parse();

    if let Err(e) = cell_sim::run(config) {
        eprintln!("Problem: {}", e);
        process::exit(1);
    };
}
