use std::collections::{HashSet,BinaryHeap};
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


fn _eliminate(puzzle: &mut Vec<Vec<HashSet<usize>>>, min_val: &mut usize, rmin: &mut usize, cmin: &mut usize, n_ones: &mut usize,  min_heap: &mut BinaryHeap<Reverse<(usize, usize, usize)>>, pq:&mut PriorityQueue<(usize,usize), usize> ) -> bool {
    let mut changed = false;
    for i in 0..puzzle.len(){
        for j in 0..puzzle.len(){ 
            if puzzle[i][j].len() == 1{
                let nums: Vec<usize> = puzzle[i][j].iter().cloned().collect();
                for num in nums{
                    if eliminate::eliminate(puzzle, i, j, num, min_val, rmin, cmin,  n_ones, min_heap, pq) {
                        changed = true;
                    }
                    println!("_eliminate rmin = {}, cmin = {}, min_val = {}, n_ones = {}, num = {}, i={}, j={}", rmin, cmin, min_val, n_ones, num, i, j);
                }
            }
        }
        println!("");
    }
    println!("\n%%%%%%%\n");
    print::print_puzzle(puzzle);
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
    let mut rmin_old = 0;
    let mut cmin_old = 0;
    let mut min_val_old = usize::MAX;
    let mut n_ones_old = 0;
    let mut min_heap: BinaryHeap<Reverse<(usize, usize, usize)>> = BinaryHeap::new();
    let mut pq: PriorityQueue<(usize,usize), usize> = priority_queue::PriorityQueue::new();
    for i in 0..puzzle.len(){
        for j in 0..puzzle.len(){
            min_heap.push(Reverse((puzzle[i][j].len(), i, j)));
            if puzzle[i][j].len() == 1{
                n_ones_old += 1;
            }
            // pq.push((i,j), puzzle[i][j].len());
        }
    }
    // pq.push((8,7), 5);
    while _eliminate(puzzle, &mut min_val_old, &mut rmin_old, &mut cmin_old, &mut n_ones_old, &mut min_heap, &mut pq) {println!("Eliminating");}
    for (item, priority) in pq.iter() {
        println!("item = {:?}, priority = {}", item, priority);
    }
    print::print_puzzle(&puzzle);
    let mut stack: Vec<(Vec<Vec<HashSet<usize>>>, (usize,usize,usize,usize), BinaryHeap<Reverse<(usize,usize,usize)>>)> = Vec::new();
    stack.push( ( puzzle.clone(), (rmin_old, cmin_old, min_val_old, n_ones_old), min_heap));
    println!("Stack length = {}, rmin={}, cmin={}, min_val={}, n_ones={}", stack.len(), rmin_old, cmin_old, min_val_old, n_ones_old);
    while !stack.is_empty() {
        let context = stack.pop().unwrap();
        let puzzle = context.0;
        let (mut rmin, mut cmin, mut min_val, mut n_ones) = context.1;
        let mut min_heap = context.2;
        let (minimum, row_min, col_min) = min_heap.pop().unwrap().0;
        // println!("rmin = {}, cmin = {}, min_val = {}, n_ones = {}", rmin, cmin, min_val, n_ones);
        // let mut puzzle = stack.pop().unwrap();
        // let changed = _eliminate(&mut puzzle);
        // print::print_puzzle(&puzzle);
        // let (row, col, min) = mrv::mrv(&mut puzzle);

        if minimum == 0{
            println!("Invalid config reached");
            continue; // invalid config reached
        }
        // eliminate::eliminate(&mut puzzle, row, col);
        if n_ones == puzzle.len() * puzzle.len() {
            println!("Found a solution!");
            let mut n_solutions = N_SOLUTIONS.lock().unwrap();
            *n_solutions += 1;
            print::print_puzzle(&puzzle);
            continue;
        }
        // println!("got here");
        let possibilities = &puzzle[row_min][col_min];
        if possibilities.len() == 1 {
            println!("possibilities = 1, invalid");
            println!("puzzle[{}][{}] = {:?}",row_min, col_min, puzzle[row_min][col_min]);
            continue;
        }
        println!("possibilities = {:?}", possibilities);
        for num in possibilities{
            let mut new_puzzle = puzzle.clone();
            let mut new_min_heap = min_heap.clone();
            let n_ones_new = n_ones.clone();

            new_puzzle[row_min][col_min] = HashSet::new();
            new_puzzle[row_min][col_min].insert(*num);
            // println!("Trying to insert {} at ({}, {}), puzzle[{}][{}]={:?}", num, rmin, cmin, rmin, cmin, new_puzzle[rmin][cmin] );
            // while _eliminate(&mut new_puzzle, &mut min_val, &mut rmin, &mut cmin, &mut n_ones) {println!("Eliminating");}
            let test = eliminate::eliminate(&mut new_puzzle, row_min, col_min, *num, &mut min_val, &mut rmin, &mut cmin, &mut n_ones, &mut new_min_heap, &mut pq.clone());
            // println!("rmin = {} cmin= {}, min = {}, n_ones = {}, num= {}", rmin, cmin, min_val, n_ones, num);
            // println!("test = {}", test);
            // println!("rmin = {}, cmin = {}, min_val = {}, n_ones = {}", rmin, cmin, min_val, n_ones);
            // while _eliminate(&mut new_puzzle) {}
            stack.push((new_puzzle, (row_min, col_min, min_val, n_ones_new), new_min_heap ));
            // println!("Stack length = {}", stack.len());
        }
    }
}

pub fn find_and_eliminate(puzzle: &mut Vec<Vec<HashSet<usize>>>){
    search(puzzle);
    println!("num_solutions = {}",N_SOLUTIONS.lock().unwrap());
}

