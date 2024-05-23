use std::collections::HashSet;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use crate::eliminate;
use crate::min_rem_val as mrv;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::print;
// use rayon::iter::PanicFuse;

lazy_static! {
    static ref N_SOLUTIONS: Mutex<usize> = Mutex::new(0);
}


fn _eliminate(puzzle: &mut Vec<Vec<HashSet<usize>>>, n_ones: &mut usize, pq:&mut PriorityQueue<(usize,usize), Reverse<usize>>, invalid: &mut bool ) -> bool {
    let mut changed = false;
    for i in 0..puzzle.len(){
        for j in 0..puzzle.len(){ 
            if puzzle[i][j].len() == 1{
                let nums: Vec<usize> = puzzle[i][j].iter().cloned().collect();
                for num in nums{
                    if eliminate::eliminate(puzzle, i, j, num,  n_ones, pq, invalid) {
                        changed = true;
                    }
                }
            }
        }
        // println!("");
    }
    return changed;
}

fn is_valid(puzzle: &Vec<Vec<HashSet<usize>>>) -> bool {
    for row in 0..puzzle.len() {
        for col in 0..puzzle.len() {
            if !is_valid_cell(&puzzle, row, col) {
                return false;
            }
        }
    }
    true
}

fn is_valid_cell(puzzle: &Vec<Vec<HashSet<usize>>>, row: usize, col: usize) -> bool {

    let num = puzzle[row][col].iter().next().unwrap();
    // Check row constraint
    for c in 0..puzzle.len() {
        if c != col && puzzle[row][c].contains(num) {
            return false;
        }
    }
    // Check column constraint
    for r in 0..puzzle.len() {
        if r != row && puzzle[r][col].contains(num) {
            return false;
        }
    }
    // Check box constraint
    let box_size = (puzzle.len() as f64).sqrt() as usize;
    let box_row = row / box_size * box_size;
    let box_col = col / box_size * box_size;
    for r in box_row..box_row + box_size {
        for c in box_col..box_col + box_size {
            if r != row && c != col && puzzle[r][c].contains(num) {
                return false;
            }
        }
    }
    true
}

pub fn search(puzzle: &mut Vec<Vec<HashSet<usize>>>) {
    let mut invalid: bool = false;
    let size = puzzle.len()*puzzle.len();
    let mut n_ones_old = 0;
    let mut pq: PriorityQueue<(usize,usize), Reverse<usize>> = priority_queue::PriorityQueue::new();
    for i in 0..puzzle.len(){
        for j in 0..puzzle.len(){
            if puzzle[i][j].len() == 1{
                n_ones_old += 1;
            }
        }
    }
    while _eliminate(puzzle,  &mut n_ones_old, &mut pq, &mut invalid) {println!("Eliminating");}
    print::print_puzzle(&puzzle);

    //context.0 = puzzle, context.1 = pq, context.2 = n_ones
    let mut stack: Vec<( Vec<Vec<HashSet<usize>>>, PriorityQueue<(usize,usize), Reverse<usize>>, usize )> = Vec::new();

    stack.push( ( puzzle.clone(), pq , n_ones_old) );
    while !stack.is_empty() {

        let context = stack.pop().unwrap();
        let puzzle = context.0;
        let mut pq = context.1;
        let mut n_ones = context.2;

        if n_ones == size{
            println!("Found a solution! #ones = {}", n_ones);
            let mut n_solutions = N_SOLUTIONS.lock().unwrap();
            *n_solutions += 1;
            print::print_puzzle(&puzzle);
            continue;
        }
        let row_min ;
        let col_min ;
        // let minimum = pq.pop().unwrap();
        if let Some((key, Reverse(priority))) = pq.pop() {
            // Now you can use `key` and `priority`
            if priority == 0 {
                // println!("Invalid config reached priority = 0");
                continue; // invalid config reached
            }
            if priority == usize::MAX {
                // println!("Invalid config reached priority = MAX");
                continue; // invalid config reached
            } else{
                row_min = key.0;
                col_min = key.1;
                // println!("PRIORITY = {}", priority);
            }
        } else {
            // println!("Priority queue is empty");
            continue;
        }
        
        let possibilities = &puzzle[row_min][col_min];

        if possibilities.len() == 1 {
            // println!("possibilities = 1, invalid");
            // println!("puzzle[{}][{}] = {:?}",row_min, col_min, puzzle[row_min][col_min]);
            continue;
        }

        // println!("possibilities = {:?}", possibilities);
        for num in possibilities{
            let mut new_puzzle = puzzle.clone();
            let mut n_ones_new = n_ones.clone();
            let mut new_pq = pq.clone();
            n_ones_new += 1;
            if n_ones_new == size{
                // println!("Found a solution! #ones = {}", n_ones_new);
                let mut n_solutions = N_SOLUTIONS.lock().unwrap();
                *n_solutions += 1;
                print::print_puzzle(&new_puzzle);
                continue;
            }    
            // println!("Trying to insert {} at puzzle[{}][{}], #ones={}", num, row_min, col_min, n_ones_new);
            new_puzzle[row_min][col_min] = HashSet::new();
            new_puzzle[row_min][col_min].insert(*num);
            eliminate::eliminate(&mut new_puzzle, row_min, col_min, *num, &mut n_ones_new, &mut new_pq, &mut invalid);
            if invalid {
                // println!("Invalid config reached invalid = true");
                invalid = false;
                continue;
            }
            stack.push((new_puzzle, new_pq, n_ones_new ));
        }
    }
}

pub fn find_and_eliminate(puzzle: &mut Vec<Vec<HashSet<usize>>>){
    search(puzzle);
    println!("num_solutions = {}",N_SOLUTIONS.lock().unwrap());
}