use std::io::{BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;
use std::collections::VecDeque;

fn read_input(filename: &str) -> Result<Vec<Vec<i8>>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
                  .and_then(|v: String| Ok(v.into_bytes().iter().map(|x| return (x - 48) as i8).collect()))).collect()
}

fn get_neighbors(input: &Vec<Vec<i8>>, i: usize, j: usize) -> (i8, i8, i8, i8) {
    let left = match j > 0 {
        true => input[i][j - 1],
        false => 10,
    };
    let right = match j < input[0].len() - 1 {
        true => input[i][j + 1],
        false => 10,
    };
    let top = match i > 0 {
        true => input[i - 1][j],
        false => 10,
    };
    let bottom = match i < input.len() - 1 {
        true => input[i + 1][j],
        false => 10,
    };
    (left, right, top, bottom)
}

fn count_valleys(input: &Vec<Vec<i8>>) -> (u64, Vec<(usize, usize)>) {
    let mut res = 0;
    let mut valley_points = vec![];
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let val = input[i][j];
            let (left, right, top, bottom) = get_neighbors(input, i, j);
            if val < left && val < right && val < top && val < bottom {
                res += (val + 1) as u64;
                valley_points.push((i, j));
            }
        }
    }
    (res, valley_points)
}

fn bfs(input: &Vec<Vec<i8>>, valley_points: &Vec<(usize, usize)>) -> usize {
    let mut visited: Vec<Vec<i16>> = vec![vec![-1; input[0].len()]; input.len()];
    
    let mut helper = |start: (usize, usize), index: usize| {
        let mut q = VecDeque::new();
        q.push_back(start);
        while !q.is_empty() {
            let (i, j) = q.pop_front().unwrap();
            visited[i][j] = index as i16;
            for (x, y) in vec![(i as i16 + 1, j as i16), (i as i16- 1, j as i16), (i as i16, j as i16 + 1), (i as i16, j as i16 - 1)] {
                if x < 0 || y < 0 {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);
                if x < input.len() && y < input[0].len() && visited[x][y] < 0 && input[x][y] > input[i][j] && input[x][y] != 9 {
                    q.push_back((x, y));
                }
            }
        }
    };
    for i in 0..valley_points.len() {
        helper(valley_points[i], i);
    }
    let mut res = vec![0; valley_points.len()];
    let visited: Vec<i16> = visited.into_iter().flatten().collect();
    for val in visited {
        if val < 0 {
            continue;
        }
        res[val as usize] += 1;
    }
    res.sort();
    res.reverse();
    res[0] * res[1] * res[2]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = read_input("example").unwrap();
        let (height, valley_points) = count_valleys(&input);
        println!("Part1: {}", height);
        println!("Part2: {}", bfs(&input, &valley_points));
    }

    #[test]
    fn actual() {
        let input = read_input("input").unwrap();
        let (height, valley_points) = count_valleys(&input);
        println!("Part1: {}", height);
        println!("Part2: {}", bfs(&input, &valley_points));
    }
}
