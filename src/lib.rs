use std::error::Error;
use std::fmt;
use std::collections::HashMap;
// use std::path;

use clap::{ Parser, ArgEnum };

mod rules;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// Number of times rule is applied
    periods: u16,

    /// State before rule is applied
    initial_state: String,
    
    /// Rule to apply
    #[clap(short, long, arg_enum, default_value_t = Rule::Rule30)]
    rule: Rule,
    
//    #[clap(short, long, parse(from_os_str))]
//    custom: Path::PathBuf
}

#[derive(ArgEnum, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Rule {
    /// Applies elementary Rule 30 to state, default value
    Rule30,

    /// Applies elementary Rule 90 to state
    Rule90,

    /// Applies elementary Rule 110 to state
    Rule110,

    /// Applies elementary Rule 184 to state
    Rule184,

//    /// Applies user-defined rule to state
//    custom(path),
}

#[derive(PartialEq, Debug)]
pub struct Rulebook {
    rules: HashMap<Vec<Cell>, u8>,
    pattern_size: usize
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub enum Cell {
    Alive,
    Dead
}

impl fmt::Display for Cell {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Alive => write!(fmt, "{}", "*"),
            Cell::Dead => write!(fmt, "{}", " ")
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let rulebook = *decode_rule(load_rule(config.rule)).unwrap();
    let mut state = str_to_cells(&config.initial_state);
     
    for i in 0..config.periods {
        print_state(i, &state);
        state = next_state(&rulebook, state);
    }

    Ok(())
}

pub fn load_rule(rule: Rule) -> &'static str {
    use crate::rules::rules;

    match rule {
        Rule::Rule30 => rules::RULE30,
        Rule::Rule90 => rules::RULE90,
        Rule::Rule110 => rules::RULE110, 
        _ => rules::RULE30
    }
}

pub fn decode_rule(rule: &str) -> Result<Box<Rulebook>, &str>  {
    let mut patterns: HashMap<Vec<Cell>, u8> = HashMap::new();

    let rule_string = String::from(rule);
    let pattern_size = String::from(&rule_string[0..4]);

    let rule_string = String::from(&rule_string[4..rule_string.len()]);
    let pattern_size = match usize::from_str_radix(&pattern_size, 2) {
        Ok(x) => x,
        Err(_) => return Err("a problem when decoding the rule occured")
    };

//    if rule_string.len() / pattern_size != rule_string.len() % pattern_size {
//        return Err("improper rule definition size - incomplete pattern")
//    }
    
    let mut pattern = vec!();
    let mut pattern_count = 0;

    for c in rule_string.chars() {
        if pattern_count == pattern_size {
            patterns.entry(pattern.clone()).or_insert(c.to_string().parse::<u8>().unwrap());
            pattern.clear(); 
            pattern_count = 0;
            continue;
        }

        match c {
            '1' => pattern.push(Cell::Alive),
            '0' => pattern.push(Cell::Dead),
            _ => return Err("rules must be defined in base 2")
        }

        pattern_count += 1;
    }

    Ok(Box::new(Rulebook { rules: patterns, pattern_size }))
}

pub fn str_to_cells(state: &String) -> Vec<Cell> {
    let mut arr = Vec::new();

    for c in state.chars() {
        if c == '*' {
            arr.push(Cell::Alive);
        } else {
            arr.push(Cell::Dead);
        }
    }

    arr
}

pub fn print_state(period: u16, state: &Vec<Cell>) {
    let mut str_state = String::from("");
    
    for c in state {
        str_state.push_str(&c.to_string());
    }

    println!("{}: {}", period, str_state);
}

pub fn next_state(rules: &Rulebook, state: Vec<Cell>) -> Vec<Cell> {
    let mut next = vec!();

    let reach = (rules.pattern_size/2) as isize;
    let state_len = state.len() as isize;

    for (i, _) in state.iter().enumerate() {
        let mut pattern = vec!();
        let ii = i as isize;

        for x in (ii-reach)..(ii+reach+1) {
            pattern.push(state[((x % state_len + state_len) % state_len) as usize].clone());
        }

        if rules.rules.contains_key(&pattern) {
            let cell = match rules.rules.get(&pattern) {
                Some(val) => match val {
                    &1 => Cell::Alive,
                    _ => Cell::Dead
                },
                None => Cell::Dead
            };

            next.push(cell);
        }
    }

    next
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn convert_str_to_cells() {
        let initial = String::from("* * ");
        let cells = vec!(Cell::Alive, Cell::Dead, Cell::Alive, Cell::Dead);

        assert_eq!(cells, str_to_cells(&initial));
    }

    #[test]
    fn convert_str_to_rule() {
        let rule = "00111011";
        let rules = Box::new(Rulebook { 
            rules: HashMap::from([
                (vec!(Cell::Alive, Cell::Dead, Cell::Alive), 1) 
            ]),
            pattern_size: 3
        });  

        assert_eq!(Ok(rules), decode_rule(rule)); 
    }
}

