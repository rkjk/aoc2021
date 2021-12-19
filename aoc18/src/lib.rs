use std::io::{BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Pl {
    Num(u32),
    Left,
    Right
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Explode,
    Split
}

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

fn parse_input(input: Vec<String>) -> Vec<Vec<Pl>> {
    input.iter().map(|val| {
        let mut res = vec![];
        for c in val.trim().chars() {
            if (c == ',') {
                continue;
            }
            else if c == '[' {
                res.push(Pl::Left);
            }
            else if c == ']' {
                res.push(Pl::Right);
            }
            else {
                res.push(Pl::Num(c.to_digit(10).unwrap()));
            }
        }
    res
    }).collect()
}

fn add(v1: &Vec<Pl>, v2: &Vec<Pl>) -> Vec<Pl> {
    let mut res = vec![Pl::Left];
    let mut v1 = v1.clone();
    let mut v2 = v2.clone();
    res.append(&mut v1);
    res.append(&mut v2);
    res.push(Pl::Right);
    res
}

fn reduce(pair: Vec<Pl>) -> Vec<Pl> {
    //let mut stack = vec![];
    let mut value = pair;
    let mut brace_count = 0;
    let mut explode_count = 0;
    let mut split_count = 0;
    for val in &value {
        if *val == Pl::Left {
            brace_count += 1;
            if brace_count >= 5 {
                explode_count += 1;
            }
        }
        else if *val == Pl::Right {
            brace_count -= 1;
        }
        else {
            if let Pl::Num(v) = val {
                if *v >= 10 {
                    split_count += 1;
                }
            }
        }
    }
    while explode_count > 0 || split_count > 0 {
        //let stack_val = stack.pop().unwrap();
        let mut new_val = vec![]; // New value of expression after performing operation

        //println!("Oldval: {:?}", value);
        //println!("explode_count: {} split_count {}", explode_count, split_count);

        if explode_count > 0 {

            let mut brace_count = 0;
            let mut last_left_ind: Option<usize> = None;
            let mut next_right_val: Option<u32> = None;
            let mut handled = false;
            let mut i = 0;
            while i < value.len() {
                let val = value[i];
                if handled {
                    if let (Pl::Num(v), Some(add)) = (val, next_right_val) {
                        let mut num = v;
                        num += add;
                        if num >= 10 {
                            //stack.push(Op::Split);
                            split_count += 1;
                        }
                        //println!("Explode op2 index {} num {}", i, num);
                        new_val.push(Pl::Num(num));
                        next_right_val = None;
                    }
                    else {
                        new_val.push(val);
                    }
                    i += 1;
                    continue;
                }
                match val {
                    Pl::Left => {
                        brace_count += 1;
                        if brace_count == 5 {
                            // Explode code here
                            //println!("Explode at index {}", i);
                            let mut left_val = 0;
                            let mut right_val = 0;
                            if let (Pl::Num(lv), Pl::Num(rv)) = (value[i + 1], value[i + 2]) {
                                left_val = lv;
                                right_val = rv;
                            }
                            // Store for later regular number update
                            next_right_val = Some(right_val);
                            // Update last_left regular value if it exists
                            match last_left_ind {
                                Some(ind) => {
                                    if let Pl::Num(vv) = value[ind] {
                                        new_val[ind] = Pl::Num(vv + left_val);
                                        //println!("Explode op1 index {} num {}", ind, vv + left_val);
                                        if vv + left_val >= 10 {
                                            //stack.push(Op::Split);
                                            split_count += 1;
                                        }
                                    }
                                },
                                None => (),
                            }
                            brace_count -= 1;
                            new_val.push(Pl::Num(0)); // Add zero and skip the coming pair
                            handled = true;
                            i += 4;
                        }
                        else {
                            new_val.push(val);
                            i += 1;
                        }
                    },
                    Pl::Right => {
                        brace_count -= 1;
                        new_val.push(val);
                        i += 1;
                    },
                    Pl::Num(v) => {
                        last_left_ind = Some(i);
                        new_val.push(Pl::Num(v));
                        i += 1;
                    }
                }
            }
            explode_count -= 1;
        }
        else {
            let mut brace_count = 0;
            let mut i = 0;
            let mut handled = false;
            while i < value.len() {
                let val = value[i];
                if handled {
                    new_val.push(val);
                    i += 1;
                    continue;
                }
                match val {
                    Pl::Right => brace_count -= 1,
                    Pl::Left => brace_count += 1,
                    Pl::Num(v) => {
                        if v >= 10 {
                            //println!("Split at index {}", i);
                            let num = (v as f32) / 2.0;
                            let split_val = [num.floor() as u32, num.ceil() as u32];
                            if brace_count == 4 {
                                //stack.push(Op::Explode);
                                explode_count += 1;
                            }
                            new_val.push(Pl::Left);
                            new_val.push(Pl::Num(split_val[0]));
                            new_val.push(Pl::Num(split_val[1]));
                            new_val.push(Pl::Right);
                            i += 1;
                            handled = true;
                            continue;
                        }
                    }
                }
                new_val.push(val);
                i += 1;
            }
            split_count -= 1;
        }
        value = new_val;
        //println!("Newval: {:?}", value);
        //println!("");
    }
    value
}

fn magnitude(input: &Vec<Pl>) -> u32 {
    let mut stack = vec![];
    for val in input {
        match val {
            Pl::Left => (),
            Pl::Num(v) => stack.push(*v),
            Pl::Right => {
                let vright = stack.pop().unwrap() * 2;
                let vleft = stack.pop().unwrap() * 3;
                stack.push(vleft + vright);
            }
        }
    }
    stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example2() {
        let mut input = parse_input(read_input("example2").unwrap());
        let mut val = input.remove(0);
        for i in 0..input.len() {
            let val2 = input.remove(0);
            val = reduce(add(&val, &val2));
            //println!("{:?}", val);
            //println!("");
        }
        println!("Part1: example2 {}", magnitude(&val));
    }

    #[test]
    fn example() {
        let mut input = parse_input(read_input("example").unwrap());
        let mut val = input.remove(0);
        for i in 0..input.len() {
            let val2 = input.remove(0);
            val = reduce(add(&val, &val2));
            //println!("{:?}", val);
            //println!("");
        }
        println!("Part1: example {}", magnitude(&val));
    }

    #[test]
    fn test_reduce() {
        let mut input = parse_input(read_input("reduce1").unwrap());
        //println!("{:?}", reduce(input.remove(0)));
        //println!("test output {:?}", reduce(input.remove(1)));
    }

    #[test]
    fn elample2() {
        let input = parse_input(read_input("example2").unwrap());
    }

    #[test]
    fn actual() {
        let mut input = parse_input(read_input("input").unwrap());
        let mut val = input.remove(0);
        for i in 0..input.len() {
            let val2 = input.remove(0);
            val = reduce(add(&val, &val2));
            //println!("{:?}", val);
            //println!("");
        }
        println!("Part1 actual: {}", magnitude(&val));
    }
}
