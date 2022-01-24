use std::fmt;

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