use std::collections::HashSet;

pub fn mrv(puzzle: &mut Vec<Vec<HashSet<usize>>>) -> (usize, usize, usize){
  let mut min = usize::MAX;
  let mut row = 0;
  let mut col = 0;
  for i in 0..puzzle.len(){
      for j in 0..puzzle.len(){
        if puzzle[i][j].len() == 0{
            return  (i, j, 0);
        }
        if puzzle[i][j].len() == 1{
            continue;
        }
        if puzzle[i][j].len() < min{
            min = puzzle[i][j].len();
            row = i;
            col = j;
        }
    }
  }
  if min == usize::MAX{
      return (usize::MAX, usize::MAX, usize::MAX);
  }
  return  (row, col, min);
}
