use std::io::{BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::cmp::{min, max};

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

fn parse_input(input: Vec<String>) -> (Vec<u8>, HashMap<(i32, i32), u8>) {
    let algo: Vec<u8> = input[0].chars().map(|v| match v {
        '.' => 0,
        '#' => 1,
        _ => panic!("Only . or #"),
    }).collect();
    let mut map = HashMap::new();
    let mut tot: i32 = (input.len() - 3) as i32;

    // Index in such a way that in the initial input, left bottom is (0, 0) and tbe input covers the first quadrant
    // Keep only pixels that are lit to save space
    for i in 2..input.len() {
        let mut j: i32 = 0;
        for c in input[i].chars() {
            let val = match c {
                '.' => 0,
                '#' => 1,
                _ => panic!("Only . or #"),
            };
            map.insert((j, tot), val);
            j += 1;
        }
        tot -= 1;
    }
    (algo, map)
}

fn enhance_x(image: &HashMap<(i32, i32), u8>, algo: &Vec<u8>, num_iter: usize) -> usize {
    let mut map = image.clone();
    //println!("Initial len: {}", map.len());
    // Get min_x, max_x and min_y, max_y where you have all zeros.ans - 2 would be the boundary.
    // For our input, the boundary would alternate between 0 and 1 every iteration

    let mut min_x: i32 = 10;
    let mut max_x: i32 = 0;
    let mut min_y: i32 = 10;
    let mut max_y: i32 = 0;
    for (kx, ky) in image.keys() {
        min_x = min(min_x, *kx);
        max_x = max(max_x, *kx);
        min_y = min(min_y, *ky);
        max_y = max(max_y, *ky);
    }
    //println!("Boundary init: {:?}", (min_x, max_x, min_y, max_y));
    for i in 0..num_iter {
        let mut ia = i as i32;
        map = enhance_once(&map, &algo, (min_x - 1 - ia, max_x + 1 + ia, min_y - 1 - ia, max_y + 1 + ia), i);
    }
    map.values().filter(|v| **v == 1).count()
}

fn enhance_once(image: &HashMap<(i32, i32), u8>, algo: &Vec<u8>, boundary: (i32, i32, i32, i32), iter: usize) -> HashMap<(i32, i32), u8> {
    let mut new_image = HashMap::new();
    let mut boundary_val = 0;
    if algo[0] == 1 && iter % 2 != 0 {
        boundary_val = 1;
    }
    //println!("iter: {} boundary: {:?} boundary_val {}", iter, boundary, boundary_val);
    let neighbors = |i, j| {
        let mut slice = [0; 9];
        let mut ind = 0;
        for kj in [1, 0, -1].iter() {
            for ki in [-1, 0, 1].iter() {
                let val = match image.get(&(i + ki, j + kj)) {
                    Some(v) => *v,
                    None => boundary_val,
                };
                slice[ind] = val;
                ind += 1;
            }
        }
        slice
    };

    for x in boundary.0..(boundary.1 + 1) {
        for y in (boundary.2)..(boundary.3 + 1) {
            let mut index_in_algo: usize = 0;
            for v in neighbors(x, y) {
                index_in_algo = (index_in_algo << 1) | (v as usize);
            }
            new_image.insert((x, y), algo[index_in_algo]);
        }
    }
    new_image
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let (algo, mut image) = parse_input(read_input("example").unwrap());
        assert_eq!(algo.len(), 512);
        println!("Part1: {}", enhance_x(&image, &algo, 2));
        println!("Part2: {}", enhance_x(&image, &algo, 50));
    }

    #[test]
    fn actual() {
        let (algo, mut image) = parse_input(read_input("input").unwrap());
        assert_eq!(algo.len(), 512);
        println!("Part1: {}", enhance_x(&image, &algo, 2));
        println!("Part2: {}", enhance_x(&image, &algo, 50));
    }
}
