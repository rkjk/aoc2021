use std::collections::HashMap;

fn simulate(input: &[u32], num_iterations: usize) -> usize {
    let mut map: HashMap<u32, usize> = HashMap::new();
    for val in input {
        *map.entry(*val).or_insert(0) += 1;
    }
    for i in 0..num_iterations {
        let mut birth_count = 0;
        let mut tmp_map = HashMap::new();
        for (k, v) in map.iter() {
            match *k == 0 {
                true => {
                    *tmp_map.entry(6).or_insert(0) += v;
                    *tmp_map.entry(8).or_insert(0) += v;
                },
                false => {
                    *tmp_map.entry(k - 1).or_insert(0) += v;
                }
            }
        }
        map = tmp_map;
    }
    map.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = [3,4,3,1,2];
        println!("Part1: {}", simulate(&input, 80));
        println!("Part2: {}", simulate(&input, 256));
    }

    #[test]
    fn actual() {
        let input = [5,4,3,5,1,1,2,1,2,1,3,2,3,4,5,1,2,4,3,2,5,1,4,2,1,1,2,5,4,4,4,1,5,4,5,2,1,2,5,5,4,1,3,1,4,2,4,2,5,1,3,5,3,2,3,1,1,4,5,2,4,3,1,5,5,1,3,1,3,2,2,4,1,3,4,3,3,4,1,3,4,3,4,5,2,1,1,1,4,5,5,1,1,3,2,4,1,2,2,2,4,1,2,5,5,1,4,5,2,4,2,1,5,4,1,3,4,1,2,3,1,5,1,3,4,5,4,1,4,3,3,3,5,5,1,1,5,1,5,5,1,5,2,1,5,1,2,3,5,5,1,3,3,1,5,3,4,3,4,3,2,5,2,1,2,5,1,1,1,1,5,1,1,4,3,3,5,1,1,1,4,4,1,3,3,5,5,4,3,2,1,2,2,3,4,1,5,4,3,1,1,5,1,4,2,3,2,2,3,4,1,3,4,1,4,3,4,3,1,3,3,1,1,4,1,1,1,4,5,3,1,1,2,5,2,5,1,5,3,3,1,3,5,5,1,5,4,3,1,5,1,1,5,5,1,1,2,5,5,5,1,1,3,2,2,3,4,5,5,2,5,4,2,1,5,1,4,4,5,4,4,1,2,1,1,2,3,5,5,1,3,1,4,2,3,3,1,4,1,1];
        println!("Part1: {}", simulate(&input, 80));
        println!("Part2: {}", simulate(&input, 256));
    }
}
