use std::io::{BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;
use std::iter::FromIterator;
use std::collections::{HashMap, HashSet};

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

fn get_score(input: &Vec<String>, part1: bool) -> u64 {
    let mut matches = HashMap::new();
    matches.insert(')', '(');
    matches.insert(']', '[');
    matches.insert('}', '{');
    matches.insert('>', '<');

    let mut scores: HashMap<char, u64> = HashMap::new();
    scores.insert(')', 3);
    scores.insert(']', 57);
    scores.insert('}', 1197);
    scores.insert('>', 25137);

    let mut scores2: HashMap<char, u64> = HashMap::new();
    scores2.insert('(', 1);
    scores2.insert('[', 2);
    scores2.insert('{', 3);
    scores2.insert('<', 4); 

    let mut openers: HashSet<char> = HashSet::new();
    for v in matches.values() {
        openers.insert(*v);
    }

    let mut values = vec![];

    let val = input.iter().fold(0, |accum, item| {
        let mut stack = vec![];
        for val in item.chars() {
            match openers.contains(&val) {
                true => stack.push(val),
                false => {
                    if stack.is_empty() || stack[stack.len() - 1] != matches[&val] {
                        match part1 {
                            true => return accum + scores[&val],
                            false => return accum,
                        }
                    }
                    stack.pop();
                }
            }
        }
        match part1 {
            true => return accum,
            false => {
                let mut res = 0;
                while !stack.is_empty() {
                    res = res * 5 + scores2[&stack.pop().unwrap()];
                }
                values.push(res);
                return accum + res;
            }
        }
    });
    if part1 {
        return val;
    }
    values.sort();
    values[values.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = read_input("example").unwrap();
        println!("Part1: {}", get_score(&input, true));
        println!("Part2: {}", get_score(&input, false));
    }
    #[test]
    fn actual() {
        let input = read_input("input").unwrap();
        println!("Part1: {}", get_score(&input, true));
        println!("Part2: {}", get_score(&input, false));
    }
}
