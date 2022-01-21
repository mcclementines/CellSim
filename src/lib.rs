use std::error::Error;
use std::env;
use std::fmt;

pub struct Config {
    periods: u16,
    initial_state: String
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let periods = match args.next() {
            Some(arg) => arg,
            None => return Err("need to define the number of periods") 
        };

        let periods = match periods.parse::<u16>() {
            Ok(num) => num,
            Err(_) => return Err("periods needs to be a valid number")
        };

        let initial_state = match args.next() {
            Some(arg) => arg,
            None => return Err("need initial sequence")
        };

        Ok(Config {
            periods,
            initial_state
        })
    }
}

#[derive(Debug, PartialEq)]
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
    let mut state = str_to_cells(&config.initial_state);
        
    for i in 0..config.periods {
        print_state(i, &state);
        state = next_state(state);
    }

    Ok(())
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

// RULE 30
pub fn next_state(state: Vec<Cell>) -> Vec<Cell> {
    let mut next = Vec::new();

    for (i, _) in state.iter().enumerate() {
        let mut before;
        let mut after;

        if i == 0 {
            before = state.len() - 1;
        } else {
            before = i - 1;
        }

        if i == state.len() - 1 {
            after = 0;
        } else {
            after = i + 1;
        }
        
        next.push(process(&state[before], &state[i], &state[after]));
    }

    next 
}

pub fn process(b: &Cell, i: &Cell, a: &Cell) -> Cell {
    use Cell::*;

    let three = (b, i, a);

    if three == (&Dead, &Dead, &Alive) {
        return Alive
    } else if three == (&Dead, &Alive, &Dead) {
        return Alive
    } else if three == (&Dead, &Alive, &Alive) {
        return Alive
    } else if three == (&Alive, &Dead, &Dead) {
        return Alive
    }

    Dead
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
}

