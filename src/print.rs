use std::collections::HashSet;

pub fn print_puzzle(puzzle: &Vec<Vec<HashSet<usize>>>){
  let size = (puzzle.len() as f64).sqrt() as usize;
  for i in 0..puzzle.len() {
      for j in 0..puzzle.len() {
          print!("{:?}\t", puzzle[i][j]);
          if (j + 1) % size == 0 {
              print!("| ");
          }
      }
      println!();
      if (i + 1) % size == 0 {
          println!("-----------------------");
      }
  }
}