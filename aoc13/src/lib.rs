use std::io::{BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;
use std::iter::FromIterator;
use std::cmp::max;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

// Reverse x and y to resemble matrix convention - first coordinate is row and second coordinate is column
fn parse_input(input: &Vec<String>) -> (Vec<Vec<bool>>, Vec<(usize, usize)>) {
    let mut flag = false;
    let mut coords = vec![];
    let mut x_max = 0;
    let mut y_max = 0;
    let mut folds = vec![];
    for val in input.iter() {
        let s = val.trim();
        if flag {
            let ss:Vec<&str> = s.split(" ").last().unwrap().split("=").collect();
            match ss[0] {
                "y" => folds.push((0, ss[1].parse::<usize>().unwrap())),
                "x" => folds.push((1, ss[1].parse::<usize>().unwrap())),
                _ => panic!("unknown input"),
            };
            continue;
        }
        if s.is_empty() {
            flag = true;
            continue;
        }
        let ss:Vec<&str> = s.split(",").collect();
        let ss0 = ss[0].parse::<usize>().unwrap();
        let ss1 = ss[1].parse::<usize>().unwrap();
        x_max = max(x_max, ss1);
        y_max = max(y_max, ss0);
        coords.push((ss1, ss0));
    }
    let mut matrix = vec![vec![false; y_max + 1]; x_max + 1];
    for val in coords {
        matrix[val.0][val.1] = true;
    }
    (matrix, folds)
}

fn fold_along_line(input: &mut Vec<Vec<bool>>, fold: (usize, usize)) {
    if fold.0 == 0 {
        for i in 1..(input.len() - fold.1) {
            for j in 0..input[0].len() {
                input[fold.1 - i][j] = input[fold.1 - i][j] | input[fold.1 + i][j];
            }
        }
        input.truncate(fold.1);
    }
    else {
        for i in 0..input.len() {
            for j in 1..(input[0].len() - fold.1) {
                input[i][fold.1 - j] = input[i][fold.1 - j] | input[i][fold.1 + j];
            }
        }
        for i in 0..input.len() {
            input[i].truncate(fold.1);
        }
    }
}

fn count_points(input: &Vec<Vec<bool>>) -> usize {
    input.iter().flatten().map(|v| match v {
        true => 1,
        false => 0
    }).sum()
}

fn pretty_print(input: &Vec<Vec<bool>>) {
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            match input[i][j] {
                true => print!("x"),
                false => print!(" "),
            }
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = read_input("example").unwrap();
        let (mut matrix, folds) = parse_input(&input);
        fold_along_line(&mut matrix, folds[0]);
        println!("Part1: {}", count_points(&matrix));
        for i in 1..folds.len() {
            fold_along_line(&mut matrix, folds[i]);
        }
        //pretty_print(&matrix);
    }

    #[test]
    fn actual() {
        let input = read_input("input").unwrap();
        let (mut matrix, folds) = parse_input(&input);
        fold_along_line(&mut matrix, folds[0]);
        println!("Part1: {}", count_points(&matrix));
        for i in 1..folds.len() {
            fold_along_line(&mut matrix, folds[i]);
        }
        pretty_print(&matrix);
    }
}
