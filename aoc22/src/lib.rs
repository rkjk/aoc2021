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

#[derive(Debug, Copy, Clone)]
struct Range {
    xr: [i32; 2],
    yr: [i32; 2],
    zr: [i32; 2],
    action: usize,
}

fn parse_input(input: Vec<String>) -> Vec<Range> {
    let mut res = vec![];
    for val in input {
        let mut tmp = vec![[0; 2], [0; 2], [0; 2]];
        let mut action = 0;
        let ss: Vec<&str> = val.trim().split(" ").collect();
        if ss[0] == "on" {
            action = 1;
        }
        let ranges: Vec<&str> = ss[1].split(",").collect();
        for (i, r) in ranges.iter().enumerate() {
            let val: Vec<&str> = r.split("=").collect::<Vec<&str>>()[1].split("..").collect();
            let num1 = val[0].parse::<i32>().unwrap();
            let num2 = val[1].parse::<i32>().unwrap();
            tmp[i][0] = num1;
            tmp[i][1] = num2;
        }
        res.push(Range {
            xr: tmp.remove(0),
            yr: tmp.remove(0),
            zr: tmp.remove(0),
            action: action
        });
    }
    res
}

fn brute_force_1(input: &Vec<Range>) -> usize {
    // 101 x 101 x 101 boolean array to hold all cubes in region
    // Note all coordinates are mapped to 0-100
    // So -50, -50, -50 -> 0, 0, 0
    // -45, 45, -20 -> 5, 95, 30
    let mut region: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; 101]; 101]; 101];
    let mut ops: Vec<Range> = vec![];
    for r in input {
        let val = [r.xr, r.yr, r.zr].concat();
        let mut flag = true;
        for v in val {
            if v < -50 || v > 50 {
                flag = false;
                break;
            }
        }
        if flag {
            ops.push(*r);
        }
    };
    for op in ops {
        for x in op.xr[0]..op.xr[1] + 1 {
            for y in op.yr[0]..op.yr[1] + 1 {
                for z in op.zr[0]..op.zr[1] + 1 {
                    //println!("{} {} {}", x, y, z);
                    region[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] = op.action;
                }
            }
        }
    }
    region.into_iter().map(|v| v.into_iter().flatten().collect::<Vec<usize>>()).flatten().sum()
}

// ON - ON -> Add OFF
fn get_intersection(r1: &Range, r2: &Range) -> Option<Range> {
    match max(r1.xr[0], r2.xr[0]) <= min(r1.xr[1] , r2.xr[1])
    && max(r1.yr[0], r2.yr[0]) <= min(r1.yr[1] , r2.yr[1])
    && max(r1.zr[0], r2.zr[0]) <= min(r1.zr[1] , r2.zr[1]) {
        false => None,
        true => Some(Range {
            xr: [max(r1.xr[0], r2.xr[0]), min(r1.xr[1] , r2.xr[1])],
            yr: [max(r1.yr[0], r2.yr[0]), min(r1.yr[1] , r2.yr[1])],
            zr: [max(r1.zr[0], r2.zr[0]), min(r1.zr[1] , r2.zr[1])],
            action: match r2.action == 0 {
                true => 1,
                false => 0
            }
        })
    }
}

fn get_total_ones(input: &Vec<Range>) -> i64 {
    let mut res = 0;
    for range in input {
        if range.action == 1 {
            res += count(range);
        }
    }
    res
}

// When comparing cuboids, if ON over OFF, or OFF over OFF, don't do anything
// If ON over ON or OFF over ON, add OFF intersection
// When comparing cuboid and intersection, if there is an intersection, add intersection with negation of the intersection's action.
fn count_total(input: &Vec<Range>) -> i64 {
    let mut intersections = vec![];
    for i in 1..input.len() {
        let cur_action = input[i].action;
        let mut new_intersections = vec![];
        // Check for intersections with all seen intervals
        for j in 0..i {
            let ac = input[j].action;
            if ac != 1 {
                continue;
            }
            if let Some(v) =  get_intersection(&input[i], &input[j]) {
                //println!("Intersection of Interval {} with interval {} => {:?}", i, j, v);
                new_intersections.push(v);
            }
        }
        
        for val in &intersections {
            if let Some(v) = get_intersection(&input[i], val) {
                //println!("Intersection of interval {} with intersection {:?}", i, val);
                new_intersections.push(v);
            }
        }
        intersections.extend(new_intersections.iter());
    }
    let mut baseline: i64 = get_total_ones(input);
    let mut net: i64 = 0;
    for val in intersections {
        if val.action == 1 {
            //println!("Adding {:?}", count(&val));
            net += count(&val);
        }
        else {
            //println!("Subtracting {:?}", count(&val));
            net -= count(&val);
        }
    }
    baseline + net
}

fn count(r: &Range) -> i64 {
    ((r.xr[1] - r.xr[0] + 1) as i64) * 
    ((r.yr[1] - r.yr[0] + 1) as i64) *
    ((r.zr[1] - r.zr[0] + 1) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = parse_input(read_input("example").unwrap());
        //println!("Part1: {}", brute_force_1(&input));
        //println!("Part2: {}", count_total(&input));
    }

    fn test_intersect() {
        let input = parse_input(read_input("example2").unwrap());
        //println!("{:?}", get_intersection(&input[0], &input[1]));
    }

    #[test]
    fn test_algo() {
        let input = parse_input(read_input("example2").unwrap());
        println!("Part2: example {}", count_total(&input));
    }

    #[test]
    fn actual() {
        let input = parse_input(read_input("input").unwrap());
        println!("Part1: {}", brute_force_1(&input));
        println!("Part2: {}", count_total(&input));
    }
}
