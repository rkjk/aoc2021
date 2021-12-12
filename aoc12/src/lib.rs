use std::io::{BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;
use std::iter::FromIterator;
use std::collections::{HashMap, HashSet};
use bimap::BiMap;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

pub struct Graph {
    node_mapping: BiMap<String, usize>,
    capital_mapping: Vec<bool>,
    graph: HashMap<usize, HashSet<usize>>,
    part2: bool,
}

impl Graph {
    pub fn new(input: &Vec<String>, part2: bool) -> Self {
        let mut node_num = 0;
        let mut node_mapping = BiMap::new();
        let mut capital_mapping = Vec::new();
        let mut graph = HashMap::new();
        for val in input.iter() {
            let edge: Vec<&str> = val.split("-").collect();
            let edge_0_map = match node_mapping.contains_left(edge[0]) {
                true => *node_mapping.get_by_left(edge[0]).unwrap(),
                false => {
                    node_mapping.insert(edge[0].to_owned(), node_num);
                    capital_mapping.push(edge[0].chars().all(|x| x.is_ascii_uppercase()));
                    node_num += 1;
                    node_num - 1
                }
            };
            let edge_1_map = match node_mapping.contains_left(edge[1]) {
                true => *node_mapping.get_by_left(edge[1]).unwrap(),
                false => {
                    node_mapping.insert(edge[1].to_owned(), node_num);
                    capital_mapping.push(edge[1].chars().all(|x| x.is_ascii_uppercase()));
                    node_num += 1;
                    node_num - 1
                }
            };
            let ch = graph.entry(edge_0_map).or_insert(HashSet::new());
            (*ch).insert(edge_1_map);
            let ch = graph.entry(edge_1_map).or_insert(HashSet::new());
            (*ch).insert(edge_0_map);
        }
        Graph {
            node_mapping: node_mapping,
            capital_mapping: capital_mapping,
            graph: graph,
            part2: part2,
        }
    }

    fn flip_part2(&mut self) {
        self.part2 = !self.part2;
    }

    // Backtracking
    fn traverse(&self, path: &mut Vec<usize>, visited_lowercase: &mut HashMap<usize, usize>, allowance_used: bool) -> usize {
        let last_node = path[path.len() - 1];
        let start_node = self.node_mapping.get_by_left("start").unwrap();
        let end_node = self.node_mapping.get_by_left("end").unwrap();
        if last_node == *end_node {
            return 1;
        }
        let mut val = 0;
        for node in &self.graph[&last_node] {
            if self.capital_mapping[*node] {
                path.push(*node);
                val += self.traverse(path, visited_lowercase, allowance_used);
                path.pop();
            }
            else {
                if !visited_lowercase.contains_key(node) {
                    visited_lowercase.insert(*node, 1);
                    path.push(*node);
                    val += self.traverse(path, visited_lowercase, allowance_used);
                    path.pop();
                    visited_lowercase.remove(node);
                }
                else if self.part2 && !allowance_used && visited_lowercase[node] == 1 && node != start_node {
                    let ch = visited_lowercase.get_mut(node).unwrap();
                    *ch += 1;
                    
                    path.push(*node);
                    val += self.traverse(path, visited_lowercase, true);
                    path.pop();
                    
                    let ch = visited_lowercase.get_mut(node).unwrap();
                    *ch -= 1;
                }
            }
        }
        val
    }

    /// Wrapper function that does a Depth First Search on the graph
    pub fn distinct_paths(&self) -> usize {
        let start_node = self.node_mapping.get_by_left("start").unwrap();
        let mut path = vec![*start_node];
        let mut visited_lowercase = HashMap::new();
        visited_lowercase.insert(*start_node, 1);
        self.traverse(&mut path, &mut visited_lowercase, false)
    }

    pub fn print(&self) {
        println!("node_mapping: {:?}", self.node_mapping);
        println!("capital mapping: {:?}", self.capital_mapping);
        println!("graph: {:?}", self.graph);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let input = read_input("example1").unwrap();
        let mut graph = Graph::new(&input, false);
        println!("Part1: {}", graph.distinct_paths());
        graph.flip_part2();
        println!("Part2: {}", graph.distinct_paths());
    }

    #[test]
    fn example2() {
        let input = read_input("example2").unwrap();
        let mut graph = Graph::new(&input, false);
        println!("Part1: {}", graph.distinct_paths());
        graph.flip_part2();
        println!("Part2: {}", graph.distinct_paths());
    }

    #[test]
    fn example3() {
        let input = read_input("example3").unwrap();
        let mut graph = Graph::new(&input, false);
        println!("Part1: {}", graph.distinct_paths());
        graph.flip_part2();
        println!("Part2: {}", graph.distinct_paths());
    }

    #[test]
    fn actual() {
        let input = read_input("input").unwrap();
        let mut graph = Graph::new(&input, false);
        println!("Part1: {}", graph.distinct_paths());
        graph.flip_part2();
        println!("Part2: {}", graph.distinct_paths());
    }
}
