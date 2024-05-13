use std::collections::HashSet;
pub fn eliminate(puzzle: &mut Vec<Vec<HashSet<usize>>>, row: usize, col: usize, num: usize) -> &mut Vec<Vec<HashSet<usize>>> {
    for i in 0..puzzle.len() {
        if puzzle[row][i].contains(&num) && puzzle[row][i].len() > 1{
            puzzle[row][i].remove(&num);
        }
    }
    for i in 0..puzzle.len() {
        if puzzle[i][col].contains(&num) && puzzle[i][col].len() > 1{
            puzzle[i][col].remove(&num);
        }
    }
    let size = (puzzle.len() as f64).sqrt() as usize;
    let start_row = row - row % size;
    let start_col = col - col % size;
    for i in 0..size {
        for j in 0..size {
            if puzzle[i + start_row][j + start_col].contains(&num) && puzzle[i + start_row][j + start_col].len() > 1{
                puzzle[i + start_row][j + start_col].remove(&num);
            }
        }
    }
    return puzzle;
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