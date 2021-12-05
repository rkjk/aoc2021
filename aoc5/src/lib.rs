use std::io::{BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;
use std::cmp::{min, max};
use std::collections::HashSet;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

type Segment= ((usize, usize), (usize, usize));

/// Consider all horizontal/vertical line segments and compute number of points of intersection
fn hv_intersect(input: &Vec<Segment>) -> usize {
    let mut points: HashSet<(usize, usize)> = HashSet::new(); // Intersecting points
    for i in 0..input.len() {
        for j in i+1..input.len() {
            //println!("Segment1 {:?}, Segment2 {:?} res: {}", input[i], input[j], res);
            let (x1, y1) = input[i].0;
            let (x2, y2) = input[i].1;

            let (x3, y3) = input[j].0;
            let (x4, y4) = input[j].1;

            // Both Vertical
            if (x1 == x2 && x3 == x4 && x1 == x3 && max(y1, y3) <= min(y2, y4)) {
                //println!("X equal Lines {:?}, {:?}", input[i], input[j]);
                for k in max(y1, y3)..(min(y2, y4)+1) {
                    points.insert((x1, k));
                }
            }
            // Both Horizontal
            if (y1 == y2 && y3 == y4 && y1 == y3 && max(x1, x3) <= min(x2, x4)) {
                //println!("Y equal Lines {:?}, {:?}", input[i], input[j]);
                for k in max(x1, x3)..(min(x2, x4)+1) {
                    points.insert((k, y1));
                }
            }
            // If one Horizontal and One vertical - x1 == x2 && y3 == y4 || x3 == x4 && y1 == y2 
            if (x1 == x2 && y3 == y4 && y3 >= min(y1, y2) && y3 <= max(y1, y2) && x1 >= min(x3, x4) && x1 <= max(x3, x4)) {
                //println!("Perpendicular 1 Lines {:?}, {:?}", input[i], input[j]);
                points.insert((x1, y3));
            }
            if (x3 == x4 && y1 == y2 && y1 >= min(y3, y4) && y1 <= max(y3, y4) && x3 >= min(x1, x2) && x3 <= max(x1, x2)) {
                //println!(" Perpendicular 2 Lines {:?}, {:?}", input[i], input[j]);
                points.insert((x3, y1));
            }
        }
    }
    //println!("{:?}", points);
    points.len()
}

fn parse_segments(input: Vec<String>) -> Vec<Segment> {
    input.iter()
        .map(|val|
            val.split("->").collect())
        .map(|ss: Vec<&str>| (ss[0].trim().split(",").collect(), ss[1].trim().split(",").collect()))
        .map(|(s0, s1): (Vec<&str>, Vec<&str>)| {
            let mut v = vec![(s0[0].parse().unwrap(), s0[1].parse().unwrap()), (s1[0].parse().unwrap(), s1[1].parse().unwrap())];
            v.sort();
            v
        })
        .map(|s: Vec<(usize, usize)>| (s[0], s[1]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = parse_segments(read_input("example").unwrap());
        println!("Part1: {}", hv_intersect(&input));
    }

    #[test]
    fn actual() {
        let input = parse_segments(read_input("input").unwrap());
        println!("Part1: {}", hv_intersect(&input));
    }
}