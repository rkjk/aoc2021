use std::io::{BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

fn parse_input(input: &Vec<String>) -> (Vec<Vec<[bool; 7]>>, Vec<Vec<[bool; 7]>>) {
    let mut unique_signals = vec![];
    let mut output = vec![];
    let bool_array = |v: &str| {
        let mut arr = [false; 7];
        for c in v.chars() {
            arr[(c as u32 - 97) as usize] = true;
        }
        arr
    };
    for val in input.iter() {
        let split_val: Vec<&str> = val.split("|").collect::<Vec<&str>>();
        unique_signals.push(split_val[0].trim().split(" ")
            .map(|v| bool_array(v)).collect());
        output.push(split_val[1].trim().split(" ")
            .map(|v| bool_array(v)).collect());
    }
    (unique_signals, output)
}

fn count_1478(output: &Vec<Vec<[bool; 7]>>) -> usize {
    output.iter()
        .map(|line| line.iter()
            .map(|val| val.iter().filter(|v| **v == true).count())
            .map(|val| val == 2 || val == 3 || val == 4 || val == 7).filter(|v1| *v1 == true).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let (unique_signals, output) = parse_input(&read_input("example").unwrap());
        println!("Part1: {}", count_1478(&output));
    }

    #[test]
    fn actual() {
        let (unique_signals, output) = parse_input(&read_input("input").unwrap());
        println!("Part1: {}", count_1478(&output));
    }
}
