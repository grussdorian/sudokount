use std::collections::HashSet;
mod input;
mod eliminate;
mod print;
mod find_and_eliminate;
mod min_rem_val;

fn main() {
    let puzzle: Vec<Vec<HashSet<usize>>> = input::take_input();
    print::print_puzzle(&puzzle);
    println!("\n\n###################################\n\n");
    let mut puzzle2 = puzzle.clone();
    find_and_eliminate::find_and_eliminate(&mut puzzle2);
    // print::print_puzzle(&puzzle2);
}