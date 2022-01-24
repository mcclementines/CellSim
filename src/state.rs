use crate::cell::Cell;
use crate::rules::Rulebook;

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

pub fn print_state(period: u16, state: &Vec<Cell>) {
    let mut str_state = String::from("");
    
    for c in state {
        str_state.push_str(&c.to_string());
    }

    println!("{}: {}", period, str_state);
}