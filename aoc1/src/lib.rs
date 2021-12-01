use std::io::{BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;

fn read_input(filename: &str) -> Result<Vec<u64>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

fn number_increases(input: &Vec<u64>) -> u64 {
    let mut res = 0;
    for i in 0..input.len() - 1 {
        if input[i] < input[i + 1]
        {
            res += 1;
        }
    }
    res
}

fn number_increases_3_window(input: &Vec<u64>) -> u64 {
    let mut res = 0;
    let mut prev = input[0] + input[1] + input[2];
    for i in 1..input.len() - 2 {
        let cur = prev - input[i - 1] + input[i + 2];
        if cur > prev {
            res += 1
        }
        prev = cur
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn actual() {
        let input = read_input("input").unwrap();
        println!("Part 1: {}", number_increases(&input));
        println!("Part 2: {}", number_increases_3_window(&input));
    }
}
