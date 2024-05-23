use std::collections::{HashSet,BinaryHeap};
use std::cmp::Reverse;
use priority_queue::PriorityQueue;

use crate::print;
pub fn eliminate(puzzle: &mut Vec<Vec<HashSet<usize>>>, row: usize, col: usize, num: usize, min: &mut usize, rmin: &mut usize, cmin: &mut usize, n_ones: &mut usize, min_heap: &mut BinaryHeap<Reverse<(usize, usize, usize)>>, pq:&mut PriorityQueue<(usize,usize), usize>  ) -> bool {
    let mut changed = false;
    if puzzle[row][col].len() != 1 {
        print!("Error: Trying to eliminate from a cell with more than one value");
        return false;
    }
    //row
    for i in 0..puzzle.len() {
        // if puzzle[row][i].contains(&num) && puzzle[row][i].len() > 1 && i != col{
        if puzzle[row][i].contains(&num) && i != col{    
            puzzle[row][i].remove(&num);
            println!("row: row ={} i={} len(row,i)={}, num removed={}", row, i, puzzle[row][i].len(), num);
            if puzzle[row][i].len() == 0 {
                *min = 0;
                *rmin = row;
                *cmin = i;
            }
            if puzzle[row][i].len() > 1 && *min > puzzle[row][i].len() {
                *rmin = row;
                *cmin = i;
                *min = puzzle[row][i].len();
            }
            changed = true;

            if puzzle[row][i].len() == 1 {
                *n_ones += 1;
                let nums: Vec<usize> = puzzle[row][i].iter().cloned().collect();
                for num in nums{
                    println!("Just before the recursive call row");
                    if eliminate(puzzle, row, i, num, min, rmin, cmin, n_ones, min_heap, pq) {
                        changed = true;
                    }
                }
            }
            if puzzle[row][i].len() != 1 {
                min_heap.push(Reverse((puzzle[row][i].len(), row, i)));
            }
        }
        pq.push((row, i), puzzle[row][i].len());
    }
    //col
    for i in 0..puzzle.len() {
        // if puzzle[i][col].contains(&num) && puzzle[i][col].len() > 1 && i != row{
            if puzzle[i][col].contains(&num) && i != row{
                puzzle[i][col].remove(&num);
                if puzzle[i][col].len() == 0 {
                    *min = 0;
                    *rmin = i;
                    *cmin = col;
                }
                if puzzle[i][col].len() > 1 && *min > puzzle[i][col].len() {
                    println!("col: i ={} col={} len(i,col)={}, num removed={}", i, col, puzzle[i][col].len(), num);
                    *rmin = row;
                    *cmin = i;
                    *min = puzzle[i][col].len();
                }
            changed = true;
            if puzzle[i][col].len() == 1 {
                *n_ones += 1;
                let nums: Vec<usize> = puzzle[i][col].iter().cloned().collect();
                for num in nums{
                    println!("Just before the recursive call col");
                    if eliminate(puzzle, i, col, num, min, rmin, cmin, n_ones, min_heap, pq) {
                        changed = true;
                    }
                }
            }
            if puzzle[i][col].len() != 1 {
                min_heap.push(Reverse((puzzle[i][col].len(), i, col)));
            }
        }
        pq.push((i, col), puzzle[i][col].len());
    }
    let size = (puzzle.len() as f64).sqrt() as usize;
    let start_row = row - row % size;
    let start_col = col - col % size;
    //box
    for i in 0..size {
        for j in 0..size {
            // if puzzle[i + start_row][j + start_col].contains(&num) && puzzle[i + start_row][j + start_col].len() > 1 && (i + start_row != row || j + start_col != col){
                if puzzle[i + start_row][j + start_col].contains(&num) && (i + start_row != row || j + start_col != col){
                    // println!("Eliminating {} from ({}, {}), set contents: {:?}", num, i + start_row, j + start_col, puzzle[i + start_row][j + start_col]);
                    puzzle[i + start_row][j + start_col].remove(&num);

                    if puzzle[i + start_row][j + start_col].len() == 0 {
                        *min = 0;
                        *rmin = i + start_row;
                        *cmin = j + start_col;
                    }

                    if puzzle[i + start_row][j + start_col].len() > 1 && *min > puzzle[i + start_row][j + start_col].len(){
                        println!("box: i ={} j={} len(i,j)={} num removed={}", i + start_row, j + start_col, puzzle[i + start_row][j + start_col].len(), num);
                        *rmin = i + start_row;
                        *cmin = j + start_col;
                        *min = puzzle[i + start_row][j + start_col].len();
                    }
                    // let x = puzzle[i + start_row][j + start_col].len();
                // if *min > x {
                //     *rmin = i + start_row;
                //     *cmin = j + start_col;
                //     *min = x;
                // }
                changed = true;
                if puzzle[i + start_row][j + start_col].len() == 1 {
                    *n_ones += 1;
                    let nums: Vec<usize> = puzzle[i + start_row][j + start_col].iter().cloned().collect();
                    for num in nums{
                        println!("Just before the recursive call box");
                        if eliminate(puzzle, i + start_row, j + start_col, num, min, rmin, cmin, n_ones, min_heap, pq) {
                            changed = true;
                        }
                    }
                }
                if puzzle[i + start_row][j + start_col].len() != 1 {
                    min_heap.push(Reverse((puzzle[i + start_row][j + start_col].len(), i + start_row, j + start_col)));
                }
            }
            pq.push((i + start_row, j + start_col), puzzle[i + start_row][j + start_col].len());
        }
    }
    return changed;
}

// use std::collections::HashSet;
// use rayon::prelude::*;

// pub fn eliminate(puzzle: &mut Vec<Vec<HashSet<usize>>>, row: usize, col: usize, num: usize) -> &mut Vec<Vec<HashSet<usize>>> {
//   let size = puzzle.len();
//   let sqrt_size = (size as f64).sqrt() as usize;
//   let start_row = row - row % sqrt_size;
//   let start_col = col - col % sqrt_size;

//   // Check the row
//   if puzzle[row].par_iter().any(|x| x.contains(&num)) {
//      puzzle[row].par_iter_mut().for_each(|x| {
//      if x.len() > 1{
//        x.remove(&num);
//      }
//      });
//   }

//   // Check the column
//   if puzzle.par_iter().any(|x| x[col].contains(&num)) {
//     puzzle.par_iter_mut().for_each(|x| {
//     if x[col].len() > 1 {
//       x[col].remove(&num);
//     }
//     });
//   }

//   // Check the box
//   for i in start_row..start_row + sqrt_size {
//     if puzzle[i][start_col..start_col + sqrt_size].par_iter().any(|x| x.contains(&num)) {
//       puzzle[i][start_col..start_col + sqrt_size].par_iter_mut().for_each(|x| {
//         if x.len() > 1 {
//           x.remove(&num);
//         }
//       });
//     }
//   }

//   return puzzle;
// }