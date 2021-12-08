use std::env;
use std::fs;

enum Cell {
    Marked(u32),
    Unmarked(u32),
}

struct Board {
    size: u32,
    cells: Vec<Cell>,
}

impl Board {
    fn new(cells: Vec<Cell>, size: u32) -> Board {
        // Returns new board from 1:1 params, meant for internal use
        Board { size, cells }
    }

    fn from_str(string: String) -> Board {
        // Returns new board from string where rows are separated by new lines
        let numbers = string.lines().map(|line| line.split(' '));
        let size = numbers.peekable().peek().unwrap().count();
        let cells = numbers.flatten().collect();

        Board::new(cells, size)
    }

    fn mark_cell(&mut self, num: u32) {
        // Marks any cell(s) with number num
    }

    fn has_bingo(&self) -> bool {
        // Return bool indicating whether or not board has bingo
        // Check rows
    }

    fn sum_unmarked(&self) -> u32 {
        // Sums up all unmarked cells and returns result
        self.cells.iter().fold(0, |acc, cell| match cell {
            Marked(num) => (),
            Unmarked(num) => acc += 1,
        })
    }
}

fn main() {
    println!("It's alive.");
}
