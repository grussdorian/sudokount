use std::collections::HashSet;
mod input;
mod eliminate;
mod print;

fn main() {
    let puzzle: Vec<Vec<HashSet<usize>>> = input::take_input();
    let size = (puzzle.len() as f64).sqrt() as usize;
    print::print_puzzle(&puzzle);
    println!("\n###################################\n");
    let mut puzzle2 = puzzle.clone();

    // for num in 1..size*size+1{
    //     let result = eliminate::eliminate(&mut puzzle2, 0, 0, num);
    //     println!("num={}, result={}",num, result);
    //     print::print_puzzle(&puzzle2);
    // }
    for i in 0..size*size{
        for j in 0..size*size{ 
            if puzzle2[i][j].len() == 1{
                let nums: Vec<usize> = puzzle2[i][j].iter().cloned().collect();
                for num in nums{
                    eliminate::eliminate(&mut puzzle2, i, j, num);
                }
            }
        }
    }
    print::print_puzzle(&puzzle2);
}