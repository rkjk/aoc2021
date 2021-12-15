use std::io::{BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;
use std::iter::FromIterator;
use std::collections::HashMap;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

fn parse_input(input: Vec<String>) -> HashMap<String, String> {
    let mut rules = HashMap::new();
    for val in input.iter() {
        let ss: Vec<&str> = val.split("->").collect();
        rules.insert(ss[0].trim().to_owned(), ss[1].trim().to_owned());
    }
    rules
}

fn brute_force(template: &Vec<String>, rules: &HashMap<String, String>, num_steps: usize) -> Vec<String> {
    let mut old_vec = template.clone();
    for _ in 0..num_steps {
        let mut new_vec = Vec::new();
        new_vec.push(old_vec[0].to_owned());
        let mut window = old_vec.windows(2);
        loop {
            match window.next() {
                Some(v) => {
                  let val = v.join(&"");
                  new_vec.push(rules[&val].to_owned());
                  new_vec.push(v[1].to_owned());  
                },
                None => break,
            };
        }
        old_vec = new_vec.clone();
    }
    old_vec
}

fn optimized(template: &Vec<String>, rules: &HashMap<String, String>, num_steps: usize) -> HashMap<String, u64> {
    let pairs: Vec<String> = template.windows(2).map(|v| v.join(&"")).collect();
    let mut old_map = HashMap::new();
    for val in pairs {
        let ch = old_map.entry(val.to_string()).or_insert(0);
        *ch += 1;
    }
    for _ in 0..num_steps {
        let mut new_map = HashMap::new();
        for (k, v) in old_map.iter() {
            let ss: Vec<char> = k.chars().collect();
            let insertion = &rules[k][..];
            let mut pair_1: String = ss[0].to_string();
            pair_1.push_str(&insertion);
            let mut pair_2: String = insertion.to_string();
            pair_2.push_str(&ss[1].to_string()[..]);

            let v1 = new_map.entry(pair_1).or_insert(0);
            *v1 += v;

            let v2 = new_map.entry(pair_2).or_insert(0);
            *v2 += v;
        }
        old_map = new_map.clone();
    }
    old_map
}

fn count_range(input: &HashMap<String, u64>, first: String, last: String) -> u64 {
    let mut counter = HashMap::new();
    for (k, v) in input.iter() {
        let ss: Vec<char> = k.chars().collect();
        let v1 = counter.entry(ss[0].to_string()).or_insert(0);
        *v1 += v;
        let v2 = counter.entry(ss[1].to_string()).or_insert(0);
        *v2 += v;
    }
    for (k, v) in counter.iter_mut() {
        *v /= 2;
    }
    let v = counter.entry(first).or_insert(0);
    *v += 1;
    let v = counter.entry(last).or_insert(0);
    *v += 1;

    let mut min_val = u64::MAX;
    let mut max_val = 0;
    for val in counter.values() {
        if *val > max_val {
            max_val = *val;
        }
        if *val < min_val {
            min_val = *val;
        }
    }
    max_val - min_val
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let template = "NNCB".chars().map(|v| v.to_string()).collect();
        let rules = parse_input(read_input("example").unwrap());
        //let output = brute_force(&template, &rules, 10);
        let output = optimized(&template, &rules, 10);
        println!("Part1: {}", count_range(&output, template[0].to_owned(), template[template.len() - 1].to_owned()));
        let output = optimized(&template, &rules, 40);
        println!("Part1: {}", count_range(&output, template[0].to_owned(), template[template.len() - 1].to_owned()));   
    }

    #[test]
    fn actual() {
        let template: Vec<String> = "PHVCVBFHCVPFKBNHKNBO".chars().map(|v| v.to_string()).collect();
        let rules = parse_input(read_input("input").unwrap());
        //let output = brute_force(&template, &rules, 10);
        let output = optimized(&template, &rules, 10);
        println!("Part1: {}", count_range(&output, template[0].to_owned(), template[template.len() - 1].to_owned()));
        let output = optimized(&template, &rules, 40);
        println!("Part1: {}", count_range(&output, template[0].to_owned(), template[template.len() - 1].to_owned()));
    }
}
