use std::io::{BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

fn parse_input(input: &Vec<String>) -> u64 {
    let mut h = 0;
    let mut d = 0;
    for val in input.iter() {
        let val = val.split(" ").collect::<Vec<&str>>();
        let (dir, mag) = (val[0], val[1]);
        let mag = mag.parse::<u64>().unwrap();
        match dir {
            "forward" =>  h += mag,
            "up" => d -= mag,
            "down" => d += mag,
            _ => panic!("Don't know what {} is", dir),
        };
    };
    h * d
}

fn parse_input_with_aim(input: &Vec<String>) -> u64 {
    let mut h = 0;
    let mut d = 0;
    let mut aim = 0;
    for val in input.iter() {
        let val = val.split(" ").collect::<Vec<&str>>();
        let (dir, mag) = (val[0], val[1]);
        let mag = mag.parse::<u64>().unwrap();
        match dir {
            "forward" =>  {
                h += mag;
                d += aim * mag;
            },
            "up" => aim -= mag,
            "down" => aim += mag,
            _ => panic!("Don't know what {} is", dir),
        };
    };
    h * d
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example()
    {
        let input = read_input("example").unwrap();
        println!("Part1: {}", parse_input(&input));
        println!("Part2: {}", parse_input_with_aim(&input))
    }
    #[test]
    fn part1()
    {
        let input = read_input("input").unwrap();
        println!("Part1: {}", parse_input(&input));
        println!("Part2: {}", parse_input_with_aim(&input))
    }
}
