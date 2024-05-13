
// use core::num;
use std::collections::HashSet;
use std::io::{self, BufRead};

pub fn take_input() -> Vec<Vec<HashSet<usize>>> {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();

  let size: usize = lines.next().unwrap().unwrap().parse().unwrap();
  let total_size = size * size * size * size;

  // let mut puzzle = vec![vec![0; size*size]; size*size];
  let mut puzzle = vec![vec![HashSet::new(); size*size]; size*size];
  let mut counter = 0;
  for i in 0..total_size {
      if counter == total_size {
          break;
      }
      let line = lines.next().unwrap().unwrap();
      let numbers: Vec<usize> = line.split_whitespace().map(|num| num.parse().unwrap()).collect();
      counter += numbers.len();
      if numbers.len() == total_size{
          for i in 0..size*size{
              for j in 0..size*size{
                  // puzzle[i][j] = numbers[i*size*size + j];
                  let item = numbers[i*size*size + j];
                  if item != 0{
                      puzzle[i][j].insert(item);
                  }
                  else{
                      for k in 1..size*size+1{
                          puzzle[i][j].insert(k);
                      }
                  }
              }
          }
      }
      else{
          for j in 0..numbers.len() {
              // puzzle[i][j] = numbers[j];
              let item = numbers[j];
              if item != 0{
                  puzzle[i][j].insert(item);
              }
              else{
                  for k in 1..size*size+1{
                      puzzle[i][j].insert(k);
                  }
              }
          }
      }
  }
  return puzzle;
}