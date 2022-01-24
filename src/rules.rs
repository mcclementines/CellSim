use std::collections::HashMap;

use crate::Cell;
use clap::ArgEnum;

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
    pub rules: HashMap<Vec<Cell>, u8>,
    pub pattern_size: usize,
}

pub fn load_rule(rule: Rule) -> &'static str {
    match rule {
        Rule::Rule30 => RULE30,
        Rule::Rule90 => RULE90,
        Rule::Rule110 => RULE110,
        Rule::Rule184 => RULE184,
    }
}

pub fn decode_rule(rule: &str) -> Result<Box<Rulebook>, &str> {
    let mut patterns: HashMap<Vec<Cell>, u8> = HashMap::new();

    let rule_string = String::from(rule);
    let pattern_size = String::from(&rule_string[0..4]);

    let rule_string = String::from(&rule_string[4..rule_string.len()]);
    let pattern_size = match usize::from_str_radix(&pattern_size, 2) {
        Ok(x) => x,
        Err(_) => return Err("a problem when decoding the rule occured"),
    };

    let mut pattern = vec![];
    let mut pattern_count = 0;

    for c in rule_string.chars() {
        if pattern_count == pattern_size {
            patterns
                .entry(pattern.clone())
                .or_insert(c.to_string().parse::<u8>().unwrap());
            pattern.clear();
            pattern_count = 0;
            continue;
        }

        match c {
            '1' => pattern.push(Cell::Alive),
            '0' => pattern.push(Cell::Dead),
            _ => return Err("rules must be defined in base 2"),
        }

        pattern_count += 1;
    }

    Ok(Box::new(Rulebook {
        rules: patterns,
        pattern_size,
    }))
}

// CONSTANTS /////////////////////////////////////////////////////////////

pub const RULE30: &str = "001111101100101010010111010100110000";
pub const RULE90: &str = "001111101101101010010111010000110000";
pub const RULE110: &str = "001111101101101110000111010100110000";
pub const RULE184: &str = "001111111100101110010111010000100000";

//////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_str_to_rule() {
        let rule = "00111011";
        let rules = Box::new(Rulebook {
            rules: HashMap::from([(vec![Cell::Alive, Cell::Dead, Cell::Alive], 1)]),
            pattern_size: 3,
        });

        assert_eq!(Ok(rules), decode_rule(rule));
    }
}
