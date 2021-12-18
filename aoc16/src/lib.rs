use std::io::{BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;
use std::iter::FromIterator;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

fn get_binary(input: &String) -> Vec<u8> {
    let mut string = "".to_string();
    for v in input.chars() {
        string.push_str(&format!("{:04b}", u32::from_str_radix(&v.to_string(), 16).unwrap()));
    }
    string.chars().map(|v| match v {
        '0' => 0,
        '1' => 1,
        _ => panic!("not binary"),
    }).collect()
}

fn binary_to_decimal(binary: &[u8]) -> usize {
    let mut res: usize = 0;
    for v in binary {
        res = (res << 1) | (*v as usize);
    }
    res.into()
}

fn control_loop(packet: &Vec<u8>) -> usize {
    let mut cur_index = 0;
    let mut version = 0;
    let (ind, number) = process_packet(packet, cur_index);
    version += number;
    cur_index = ind;
    println!("cur_version: {}", version);
    version
}

// Process packet starting at start_index: Return end_index and version of the packet
fn process_packet(packet: &Vec<u8>, start_index: usize) -> (usize, usize) {
    let mut cur_index = start_index;
    println!("Processing packet from index {}", cur_index);
    let mut version = binary_to_decimal(&packet[cur_index..(cur_index+3)]);
    cur_index += 3;
    let type_id = binary_to_decimal(&packet[cur_index..(cur_index+3)]);
    cur_index += 3;
    println!("version: {} type_id {}", version, type_id);
    // Literal value
    if type_id == 4 {
        let (ind, number) = process_literal(packet, cur_index);
        cur_index = ind;
        return (cur_index, version);
    }
    let length_type_id = packet[cur_index];
    cur_index += 1;

    // Length Type Id = 0
    if length_type_id == 0 {
        let subpacket_length = binary_to_decimal(&packet[cur_index..(cur_index + 15)]);
        cur_index += 15;
        let orig_index = cur_index;
        loop {
            let (ind, number) = process_packet(packet, cur_index);
            version += number;
            cur_index = ind;
            if cur_index - orig_index == subpacket_length {
                break;
            }
        }
        return (cur_index, version);
    }
    // Length type ID = 1
    let num_subpackets = binary_to_decimal(&packet[cur_index..(cur_index + 11)]);
    cur_index += 11;
    for _ in 0..num_subpackets {
        let (ind, number) = process_packet(packet, cur_index);
        version += number;
        cur_index = ind;
    }
    (cur_index, version)
}

fn process_literal(packet: &Vec<u8>, start_index: usize) -> (usize, usize) {
    println!("Processing literal from index: {}", start_index);
    let mut cur_index = start_index;
    let mut num = vec![];
    loop {
        let block = &packet[cur_index..(cur_index + 5)];
        num.extend(&block[1..]);
        cur_index += 5;
        if block[0] == 0 {
            break;
        }
    }
    let val = binary_to_decimal(&num);
    println!("Array: {:?} Literal: {}", num, val);
    (cur_index, val)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let strings = read_input("example").unwrap();
        let mut result = vec![];
        for (i, input) in strings.iter().enumerate() {
            let val = get_binary(input);
            println!("{:?} {}", input, val.len());
            result.push(control_loop(&val));
            //println!("Input {} {}", i, control_loop(&val));
        }
        println!("{:?}", result);
    }

    #[test]
    fn actual() {
        let input = get_binary(&read_input("input").unwrap()[0]);
        println!("Part1: {}", control_loop(&input));
    }
}
