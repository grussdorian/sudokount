use std::collections::HashSet;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::eliminate::{eliminate, _eliminate};


lazy_static! {
    static ref N_SOLUTIONS: Mutex<usize> = Mutex::new(0);
}

pub fn search(puzzle: &mut Vec<Vec<HashSet<usize>>>) {
    // Get the cell with the minimum number of possibilities
    let (row, col, _) = mrv(puzzle);

    // Get a reference to the set of possibilities for this cell
    let possibilities = &puzzle[row][col];

    // Iterate over each possibility
    for &num in possibilities {
        // Create a copy of the puzzle
        let mut new_puzzle = puzzle.clone();

        // Remove all other possibilities from this cell
        new_puzzle[row][col] = HashSet::new();
        new_puzzle[row][col].insert(num);

        // Eliminate this number from the row, column, and box
        eliminate(&mut new_puzzle, row, col, num);

        // Keep eliminating until no more eliminations are possible
        while _eliminate(&mut new_puzzle) {}

        // If the puzzle is solved, increment the counter
        if is_solved(&new_puzzle) {
            let mut n_solutions = N_SOLUTIONS.lock().unwrap();
            *n_solutions += 1;
        }
    }
}