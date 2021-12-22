use std::collections::{VecDeque, HashMap};
use std::cmp::max;

fn game(start1: u32, start2: u32) -> u32 {
    let mut player1 = true;
    let mut score1 = 0;
    let mut score2 = 0;

    let mut p1 = start1;
    let mut p2 = start2;
    
    let mut num_turns = 0;

    let mut dice = 0;
    while score1 < 1000 && score2 < 1000 {
        let mut delta = 0;
        for _ in 0..3 {
            delta += dice + 1;
            dice  = (dice + 1) % 100;
        }
        if player1 {
            p1 = (p1 + delta) % 10;
            score1 += p1 + 1;
        }
        else {
            p2 = (p2 + delta) % 10;
            score2 += p2 + 1;
        }
        player1 = !player1;
        num_turns += 1;
    }
    if score1 < 1000 {
        return score1 * num_turns * 3
    }
    else {
        return score2 * num_turns * 3
    }
}

// Fiendishly hard Part2: Essentially, simulate all the win states for both players
fn quantum_game(start1: u64, start2: u64) -> (u64, u64) {
    // Distribution of x + y + z where x belongs to [1,2,3]
    // Total 3^3 = 27 possible combinations. 7 distinct sums (3-9)
    let arr = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]; 


    let FINAL_SCORE = 21;

    // Player loop: Simulate all possible scores/positions from all previously seen (position, score) keys.
    // Value is the number of universes with that combination of (pos, score) in that iteration for this player.
    let player_loop = |map: &HashMap<(u64, u64), u64>| {
        let get_position = |i, add| {
            let mut new_val = i + add;
            if new_val > 10 {
                new_val -= 10;
            }
            new_val
        };    
        let mut num_win = 0;
        let mut num_lose = 0;
        let mut new_map = HashMap::new();
        for (k, v) in map.iter() {
            let (pos, score) = k;
            match *score >= FINAL_SCORE {
                true => num_win += v,
                false => {
                    num_lose += v;
                    for (val, quantity) in arr {
                        let npos = get_position(pos, val);
                        let ch = new_map.entry((npos, score + npos)).or_insert(0);
                        *ch += v * quantity;
                    }
                }
            }
        }
        (num_win, num_lose, new_map)
    };

    let mut wins1 = vec![]; // Number of wins in an iteration, starting from first iteration
    let mut loses1 = vec![]; // Number of losses in an iteration, starting from first iteration
    let mut wins2 = vec![];  // Same for player2, except, wins2[i] happens chronologically after wins1[i] and loses1[i]
    let mut loses2 = vec![]; // loses2[i] happens after wins1[i] and loses1[i]


    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();
    
    map1.insert((start1, 0), 1);
    map2.insert((start2, 0), 1);
    while !map1.is_empty() && !map2.is_empty() {
        let (win1, lose1, newmap1) = player_loop(&map1);
        let (win2, lose2, newmap2) = player_loop(&map2);
        wins1.push(win1);
        wins2.push(win2);
        loses1.push(lose1);
        loses2.push(lose2);
        map1 = newmap1;
        map2 = newmap2;
    }

    let mut winners1 = 0;
    let mut winners2 = 0;

    // Since Player2 plays after player1, wins in an iteration are measured against losses in the same iteration for Player1 (played just before player2)
    for i in 0..wins2.len() {
        winners2 += wins2[i] * loses1[i];
    }

    // Tricky part: Player1's wins in an iteration are measured against Player2's wins in the previous iteration, 
    // since player1 plays first, wins2[i], loses2[i] have not yet happened yet.
    for i in 0..wins1.len() {
        if wins1[i] != 0 {
            winners1 += wins1[i] * loses2[i - 1];
        }
    }
    (winners1, winners2)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        println!("Part1: {}", game(3, 7));
        println!("Part2: {:?}", quantum_game(4, 8));
    }

    #[test]
    fn actual() {
        println!("Part1: {}", game(6, 8));
        println!("Part2: {:?}", quantum_game(7, 9));
    }
}
