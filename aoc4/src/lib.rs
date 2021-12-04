use std::io::{BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

/// Extract numbers read during bingo
fn get_numbers(input: &Vec<String>) -> Vec<usize> {
    input[0].split(",").map(|val| val.parse().unwrap()).collect()
}

/// Extract Bingo matrices
/// Use Rust const generics to generalize methods on the size of the bingo matrix
/// Although in this case, both parts operate on 5x5, it is a cool feature 
fn get_boards<const T: usize>(input: &Vec<String>) -> Vec<[[usize; T]; T]> {
    let mut boards: Vec<[[usize; T]; T]> = vec![];
    for i in (2..input.len()).step_by(T+1) {
        let mut board =  [[0; T]; T];
        for j in i..i+T {
            let tmp: Vec<&str> = input[j].trim().split_whitespace().collect();
            for k in 0..T {
                board[j - i][k] = tmp[k].parse().unwrap();
            }
        }
        boards.push(board);
    }
    boards
}

/// Conduct simulation and compute winning score
fn conduct_bingo<const T: usize>(numbers: &Vec<usize>, mut boards: Vec<[[usize; T]; T]>, break_early: bool) -> usize {
    let mut winner = 0;
    let mut winning_num = 0;
    let mut last = 0;
    let mut last_winning_num = 0;

    // Wrap the bingo simulation in a closure that allows early return for part 1
    let mut bingo_loop = || {
        let mut row_done: Vec<[usize; T]> = vec![[T; T]; boards.len()];
        let mut col_done = row_done.clone();
        let mut board_done = vec![false; boards.len()];
        for num in numbers.iter() {
            for (ind, board) in boards.iter_mut().enumerate() {
                for i in 0..T {
                    for j in 0..T {
                        if board[i][j] == *num {
                            row_done[ind][i] -= 1;
                            col_done[ind][j] -= 1;
                            board[i][j] = usize::MAX;
                            if row_done[ind][i] == 0 || col_done[ind][j] == 0 {
                                board_done[ind] = true;
                                if board_done.iter().all(|&val| val) {
                                    last = ind;
                                    last_winning_num = *num;
                                    return;
                                }
                                winner = ind;
                                winning_num = *num;
                                if break_early {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    };
    bingo_loop();
    if (!break_early) {
        winner = last;
        winning_num = last_winning_num;
    }
    let mut unmarked_sum = 0;
    for i in 0..T {
        for j in 0..T {
            if boards[winner][i][j] != usize::MAX {
                unmarked_sum += boards[winner][i][j];
            }
        }
    }
    unmarked_sum * winning_num
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = read_input("example").unwrap();
        let numbers = get_numbers(&input);
        let boards = get_boards::<5>(&input);
        println!("Part1: {}", conduct_bingo::<5>(&numbers, boards.clone(), true));
        println!("Part2: {}", conduct_bingo::<5>(&numbers, boards.clone(), false));
    }

    #[test]
    fn actual() {
        let input = read_input("input").unwrap();
        let numbers = get_numbers(&input);
        let boards = get_boards::<5>(&input);
        println!("Part1: {}", conduct_bingo::<5>(&numbers, boards.clone(), true));
        println!("Part2: {}", conduct_bingo::<5>(&numbers, boards.clone(), false));
    }
}