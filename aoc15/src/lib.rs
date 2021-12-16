use std::io::{BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;
use std::iter::FromIterator;
use std::cmp::min;
use std::collections::{VecDeque, BinaryHeap};

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<i32>> {
    input.iter().map(|row| row.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()).collect()
}

fn dijkstra(grid: &Vec<Vec<i32>>) -> i32 {
    let mut min_cost = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
    min_cost[0][0] = 0;

    let neighbors = |i, j| {
        let mut res = vec![];
        if i < grid.len() - 1 {
            res.push((i + 1, j));
        }
        if i > 0 {
            res.push((i - 1, j));
        }
        if j > 0 {
            res.push((i, j - 1));
        }
        if j < grid[0].len() - 1 {
            res.push((i, j + 1));
        }
        res
    };

    let mut heap = BinaryHeap::new();
    heap.push((0, (0, 0)));
    
    while !heap.is_empty() {
        let (cost, coord) = heap.pop().unwrap();
        let cost: i32 = cost * -1;
        let (i, j) = coord;

        for (x, y) in neighbors(i, j) {
            if cost + grid[x][y] < min_cost[x][y] {
                min_cost[x][y] = cost + grid[x][y];
                heap.push((-1 * min_cost[x][y], (x, y)));
            }
        }
    }
    min_cost[grid.len() - 1][grid[0].len() - 1]
}

fn bfs(grid: &Vec<Vec<i32>>) -> i32 {
    let mut min_cost = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
    min_cost[0][0] = 0;
    let mut q = VecDeque::new();
    q.push_back(((0, 1), 0)); // Coordinate and current cost
    q.push_back(((1, 0), 0));
    let neighbors = |i, j| {
        let mut res = vec![];
        if i < grid.len() - 1 {
            res.push((i + 1, j));
        }
        if i > 0 {
            res.push((i - 1, j));
        }
        if j > 0 {
            res.push((i, j - 1));
        }
        if j < grid[0].len() - 1 {
            res.push((i, j + 1));
        }
        res
    };
    while !q.is_empty() {
        let (coord, cost) = q.pop_front().unwrap();
        let (i, j) = coord;
        if cost + grid[i][j] >= min_cost[i][j] {
            continue;
        }
        min_cost[i][j] = cost + grid[i][j];
        for (x, y) in neighbors(i, j) {
            q.push_back(((x, y), min_cost[i][j]));
        }
    }
    min_cost[grid.len() - 1][grid[0].len() - 1]
}

fn get_tiles(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let M = grid.len();
    let N = grid[0].len();
    let mut new_grid = vec![vec![0; N * 5]; M * 5];
    for i in 0..new_grid.len() {
        for j in 0..new_grid[0].len() {
            let mut val = grid[i % M][j % N] + (i / M) as i32 + (j / N) as i32;
            if val >= 10 {
                val -= 9;
            }
            new_grid[i][j] = val;
        }
    }
    new_grid
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let grid = parse_input(&read_input("example").unwrap());
        println!("Part1: {}", bfs(&grid));
        let new_grid = get_tiles(&grid);
        //println!("Part2: {}", bfs(&new_grid));
        println!("Part2 Dijkstra: {}", dijkstra(&new_grid));
    }

    #[test]
    fn actual() {
        let grid = parse_input(&read_input("input").unwrap());
        println!("Part1: {}", bfs(&grid));
        let new_grid = get_tiles(&grid);
        //println!("Part2: {}", bfs(&new_grid));
        println!("Part2 Dijkstra: {}", dijkstra(&new_grid));
    }
}
