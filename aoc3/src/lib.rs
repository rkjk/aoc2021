use std::io::{BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

/// Compute the most common bit in each position
/// If 1 is most common, res[i] > 0
/// If 0 is most common, res[i] < 0
/// Equally matched, res[i] = 0
fn compute_dominant_bit(input: &Vec<String>, mask: &Vec<bool>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![0; input[0].len()];
    for (i, val) in input.iter().enumerate() {
        if !mask[i] { continue }; // If mask is not set at current index, don't consider the number
        for (index, digit) in val.chars().enumerate() {
            match digit {
                '0' => res[index] -= 1,
                '1' => res[index] += 1,
                _ => (),
            }
        }
    }
    res
}

fn compute_rating(input: &Vec<String>, oxygen: bool) -> u64 {
    let mut mask = vec![true; input.len()]; // Start computation with all numbers included, progresively whittled down
    let new_input: Vec<&[u8]> = input.iter().map(|val| val.as_bytes()).collect::<Vec<&[u8]>>();
    let mut j: usize = 0; // Current bit index
    while mask.iter().map(|val| if *val {1} else {0}).sum::<u32>() > 1 {
        let res = compute_dominant_bit(input, &mask);
        for i in 0..mask.len() {
            let val = mask[i];
            if oxygen && val && ( (new_input[i][j] == b'1' && res[j] < 0) || (new_input[i][j] == b'0' && res[j] >= 0) ) {
                mask[i] = !mask[i]
            }
            if !oxygen && val && ( (new_input[i][j] == b'1' && res[j] >= 0) || (new_input[i][j] == b'0' && res[j] < 0) ) {
                mask[i] = !mask[i];
            }
        }
        j += 1;
    }
    let mut rating: u64 = 0;
    for (i, val) in mask.iter().enumerate() {
        if *val {
            for c in new_input[i].iter() {
                match c {
                    b'0' => rating = rating << 1,
                    b'1' => rating = (rating << 1) | 1,
                    _ => (),
                }
            }
        }
    }
    rating
}

fn compute_power(input: &Vec<String>) -> u64 {
    let res = compute_dominant_bit(input, &vec![true; input.len()]);
    let mut gamma: u64 = 0;
    let mut epsilon: u64 = 0;

    for digit in res.iter() {
        match *digit >= 0 {
            true => {
                gamma = (gamma << 1) | 1;
                epsilon = epsilon << 1;
            },
            false => {
                gamma = gamma << 1;
                epsilon = (epsilon << 1) | 1;
            },
        }
    }
    gamma * epsilon
}

fn compute_life_support_rating(input: &Vec<String>) -> u64 {
    let oxygen = compute_rating(input, true);
    let carbon = compute_rating(input, false);
    oxygen * carbon
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = read_input("example").unwrap();
        println!("Part1: {}", compute_power(&input));
        println!("Part2: {}", compute_life_support_rating(&input));
    }

    #[test]
    fn actual() {
        let input = read_input("input").unwrap();
        println!("Part1: {}", compute_power(&input));
        println!("Part2: {}", compute_life_support_rating(&input));
    }
}
