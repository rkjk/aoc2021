use std::io::{BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;
use std::collections::VecDeque;

fn read_input(filename: &str) -> Vec<Vec<u8>> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    let mut input = f.lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
                  .and_then(|v: String| Ok(v.into_bytes().iter().map(|x| return (x - 48) as u8).collect()))).collect::<Result<Vec<Vec<u8>>, _>>().unwrap();
    // Augment Row of zeros and Column of zeros
    input.insert(0, vec![0; input[1].len()]);
    input.push(vec![0; input[0].len()]);
    for val in input.iter_mut() {
        val.insert(0, 0);
        val.push(0);
    }
    input
}

fn count_flashes(input: &Vec<Vec<u8>>, num_iter: usize, part1: bool) -> u64 {
    let mut old_state = input.clone();
    let neighbors = |i: usize, j: usize| -> Vec<(usize, usize)> {
        vec![(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1), 
            (i + 1, j + 1), (i + 1, j - 1), (i - 1, j + 1), (i - 1, j - 1)]
    };
    let mut update_neighbors = |i, j, old_state: &mut Vec<Vec<u8>>, flashed: &Vec<Vec<bool>>| {
        for (x, y) in neighbors(i, j) {
            if x == 0 || x == input.len() - 1 || y == 0 || y == input[0].len() - 1 || flashed[x][y] {
                continue;
            }
            old_state[x][y] += 1;
        }
    };
    let mut num_flashes = 0;
    let mut iter = 0;
    loop {
        if part1 && iter == 100 {
            break;
        }
        // Update everything by 1
        for j in 1..(input.len() - 1) {
            for k in 1..(input[0].len() - 1) {
                old_state[j][k] += 1;
            }
        }
        // Propagate flashes until there are no new flashes
        let mut flashed = vec![vec![false; input[0].len()]; input.len()];
        loop {
            let mut num_changes = 0;
            for j in 1..(input.len()) {
                for k in 1..(input.len()) {
                    if old_state[j][k] >= 10 {
                        old_state[j][k] = 0;
                        flashed[j][k] = true;
                        update_neighbors(j, k, &mut old_state, &flashed);
                        num_changes += 1;
                    }
                }
            }
            num_flashes += num_changes;
            if num_changes == 0 {
                break;
            }
        }
        // Count total number of flashes
        let mut tot_flash = 0;
        for i in 1..(input.len() - 1) {
            for j in 1..(input[0].len() - 1) {
                tot_flash += match flashed[i][j] {
                    true => 1,
                    false => 0
                };
            }
        }
        if tot_flash == (input.len() - 2) * (input[0].len() - 2) {
            return iter + 1;
        }
        iter += 1;
    }
    num_flashes
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let mut input = read_input("example");
        println!("Part1: {}", count_flashes(&input, 100, true));
        println!("Part2: {}", count_flashes(&input, 100, false));
    }

    #[test]
    fn actual() {
        let mut input = read_input("input");
        println!("Part1: {}", count_flashes(&input, 100, true));
        println!("Part2: {}", count_flashes(&input, 100, false));
    }
}
