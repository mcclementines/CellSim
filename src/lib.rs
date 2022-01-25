mod cell;
mod rules;
mod state;

use std::error::Error;
// use std::path;

use clap::Parser;

use cell::Cell;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// Number of times rule is applied
    periods: u16,

    /// State before rule is applied
    initial_state: String,

    /// Rule to apply
    #[clap(short, long, arg_enum, default_value_t = rules::Rule::Rule30)]
    rule: rules::Rule,

    /// Apply custom rule
    #[clap(short, long)]
    custom: Option<String>,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let rule = match config.custom {
        Some(x) => x,
        None => String::from(rules::load_rule(config.rule)),
    };

    let rulebook = *rules::decode_rule(&rule).unwrap();
    let mut state = cell::str_to_cells(&config.initial_state);

    for i in 0..config.periods {
        state::print_state(i, &state);
        state = state::next_state(&rulebook, state);
    }

    Ok(())
}
