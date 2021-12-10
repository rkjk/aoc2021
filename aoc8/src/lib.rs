use std::io::{BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

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

fn evaluate(mut input: Vec<Vec<[bool; 7]>>, output: &Vec<Vec<[bool; 7]>>) -> usize {
    let mut map = HashMap::new();
    map.insert([false, false, true, false, false, true, false], 1); // cf - 1
    map.insert([true, false, true, false, false, true, false], 7); // acf - 7
    map.insert([false, true, true, true, false, true, false], 4); // bcdf - 4
    map.insert([true, false, true, true, true, false, true], 2); // acdeg - 2
    map.insert([true, false, true, true, false, true, true], 3); // acdfg - 3
    map.insert([true, true, false, true, false, true, true], 5); // abdfg - 5
    map.insert([true, true, true, false, true, true, true], 0); // abcefg - 0
    map.insert([true, true, false, true, true, true, true], 6); // abcefg - 6
    map.insert([true, true, true, true, false, true, true], 9); // abcefg - 9
    map.insert([true, true, true, true, true, true, true], 8); // abcefg - 8
    let mut total = 0;
    for (val1, out1) in input.iter_mut().zip(output.iter()) {
        let mapping = evaluate_row(val1);
        let mut res = 0;
        for v in out1 {
            res = res * 10 + mapping[v];
        }
        total += res;
    }
    total
}

fn evaluate_row(input: &Vec<[bool; 7]>) -> HashMap<[bool; 7], usize> {
    // Sort by number of true -> order [2, 3, 4, 5, 5, 5, 6, 6, 6, 7]
    let mut sorted_input: Vec<[bool; 7]> = input.clone();
    sorted_input.sort_by_key(|val| val.iter().filter(|v| **v == true).count());
    let mut mapping: HashMap<[bool; 7], usize> = HashMap::new();

    // Length 2 => 1
    mapping.insert(sorted_input[0], 1);

    // Length 3 => 7
    mapping.insert(sorted_input[1], 7);

    // Length 4 is 4
    mapping.insert(sorted_input[2], 4);

    // Length 7 is 8
    mapping.insert(sorted_input[9], 8);

    let match_helper = |val1: &[bool; 7], val2: &[bool; 7]| -> usize {
        val1.iter().zip(val2.iter()).map(|(a, b)| match *a == true && *b == true {
            true => 1,
            false => 0
        }).sum()
    };

    // Among length 5, if 2 letters match with 1 => 3 else if 3 letters match with 4 => 5, else 2
    for val in sorted_input[3..6].iter() {
        let mut num_matches_with_1 = match_helper(val, &sorted_input[0]); // Index 0 is 1
        let mut num_matches_with_4 = match_helper(val, &sorted_input[2]); // Index 2 is 4
        if num_matches_with_1 == 2 {
            mapping.insert(*val, 3);
        } else if num_matches_with_4 == 3 {
            mapping.insert(*val, 5);
        } else {
            mapping.insert(*val, 2);
        }
    }

    // Among length 6, if 1 letter matches with 1 it is 6, else if 4 letters match with 4 it is 9, else 0
    for val in sorted_input[6..9].iter() {
        let mut num_matches_with_1 = match_helper(val, &sorted_input[0]);
        let mut num_matches_with_4 = match_helper(val, &sorted_input[2]);
        if num_matches_with_1 == 1 {
            mapping.insert(*val, 6);
        } else if num_matches_with_4 == 4 {
            mapping.insert(*val, 9);
        } else {
            mapping.insert(*val, 0);
        }
    }
    mapping
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let (unique_signals, output) = parse_input(&read_input("example").unwrap());
        println!("Part1: {}", count_1478(&output));
        println!("Part2: {}", evaluate(unique_signals, &output));
    }

    #[test]
    fn actual() {
        let (unique_signals, output) = parse_input(&read_input("input").unwrap());
        println!("Part1: {}", count_1478(&output));
        println!("Part2: {}", evaluate(unique_signals, &output));
    }
}
