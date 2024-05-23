use std::collections::HashSet;
use std::cmp::Reverse;
use priority_queue::PriorityQueue;

use crate::print;
pub fn eliminate(puzzle: &mut Vec<Vec<HashSet<usize>>>, row: usize, col: usize, num: usize, n_ones: &mut usize, pq:&mut PriorityQueue<(usize,usize), Reverse<usize> >, invalid: &mut bool ) -> bool {
    
    if *invalid {
        return false;
    }
    
    let mut changed = false;

    if puzzle[row][col].len() != 1 {
        // print!("Error: Trying to eliminate from a cell with more than one value");
        return false;
    }
    //row
    for i in 0..puzzle.len() {
        if puzzle[row][i].contains(&num) && i != col{    
            puzzle[row][i].remove(&num);
            if puzzle[row][i].len() == 0 {
                // println!("Error: Empty set in row col {},{}", row, i);
                pq.push((row, i), Reverse(0)); // (key, priority)
                *invalid = true;
                return false;
            }
            changed = true;

            if puzzle[row][i].len() == 1 {
                *n_ones += 1;
                let nums: Vec<usize> = puzzle[row][i].iter().cloned().collect();
                for num in nums{
                    // println!("Just before the recursive call row");
                    // pq.change_priority(&(row,i), Reverse(usize::MAX));
                    pq.remove(&(row, i));
                    if eliminate(puzzle, row, i, num,  n_ones, pq, invalid) {
                        changed = true;
                    } else{
                        changed = false;
                    }
                }
            }else{
                pq.push((row, i), Reverse(puzzle[row][i].len()));
            }
        }
        if puzzle[row][i].len() == 0 {
            // println!("Error: Empty set in row col {},{}", row, i);
        }
    }
    //col
    for i in 0..puzzle.len() {
            if puzzle[i][col].contains(&num) && i != row{
                puzzle[i][col].remove(&num);
                changed = true;
                if puzzle[i][col].len() == 0 {
                    // println!("Error: Empty set in row col {},{}", i, col);
                    pq.push((i, col), Reverse(0)); // (key, priority)
                    *invalid = true;
                    return false;
                }
                if puzzle[i][col].len() == 1 {
                    *n_ones += 1;
                    let nums: Vec<usize> = puzzle[i][col].iter().cloned().collect();
                    for num in nums{
                        pq.remove(&(i, col));
                        if eliminate(puzzle, i, col, num, n_ones, pq, invalid) {
                            changed = true;
                        }else{
                            changed = false;
                        }
                    }
                }else{
                    pq.push((i, col), Reverse(puzzle[i][col].len()));
                }
        }
        if puzzle[i][col].len() == 0 {
            // println!("Error: Empty set in row col {},{}", i, col);
        }
    }
    let size = (puzzle.len() as f64).sqrt() as usize;
    let start_row = row - row % size;
    let start_col = col - col % size;
    //box
    for i in 0..size {
        for j in 0..size {
                if puzzle[i + start_row][j + start_col].contains(&num) && (i + start_row != row || j + start_col != col){
                    puzzle[i + start_row][j + start_col].remove(&num);
                    changed = true;
                    if puzzle[i + start_row][j + start_col].len() == 0 {
                        // println!("Error: Empty set in row col {},{}", i + start_row, j + start_col);
                        pq.push((i + start_row, j + start_col), Reverse(0)); // (key, priority)
                        *invalid = true;
                        return false;
                    }
                    if puzzle[i + start_row][j + start_col].len() == 1 {
                        *n_ones += 1;
                        let nums: Vec<usize> = puzzle[i + start_row][j + start_col].iter().cloned().collect();
                        for num in nums{
                            pq.remove(&(i + start_row, j + start_col));
                            if eliminate(puzzle, i + start_row, j + start_col, num, n_ones, pq, invalid) {
                                changed = true;
                            } else{
                                changed = false;
                            }
                        }
                    }else{
                        pq.push((i + start_row, j + start_col), Reverse(puzzle[i + start_row][j + start_col].len()));
                    }
            }
            if puzzle[i + start_row][j + start_col].len() == 0 {
                // println!("Error: Empty set in row col {},{}", i + start_row, j + start_col);
            }
        }
    }
    return changed;
}
