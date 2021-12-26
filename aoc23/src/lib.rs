use std::collections::{HashMap, BinaryHeap};
use std::cmp::{Ordering, min};

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
enum Pod {
    A1,
    A2,
    B1,
    B2,
    C1,
    C2,
    D1,
    D2
}

#[derive(Debug, Eq, Hash, Copy, Clone)]
struct State {
    rooms: [Option<Pod>; 8],
    hallway: [Option<Pod>; 7]
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        Ordering::Less
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ordering::Less)
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        (self.rooms, self.hallway) == (other.rooms, other.hallway)
    }
}

// Hallway marking
// Hallway space above rooms are out of bounds. Starting from left, there are 7 spaces numbered 0 - 6.
// Room spaces are in order left to right, top to bottom. So A-Top, A-Bottom, B-Top, .... D-Bottom. Numbered 0 - 7.
// #############
// #...........#
// ###D#B#D#B###
//   #C#A#A#C#
//   #########
fn dijkstra(mut start: [Option<Pod>; 8]) -> i64 {
    // Cost to get from Room space to Hallway space
    let hall_cost = [
        [3, 2, 2, 4, 6, 8, 9], // A-Top to Hallway
        [4, 3, 3, 5, 7, 9, 10], // A-Bottom to Hallway
        [5, 4, 2, 2, 4, 6, 7], // B-Top to hallway
        [6, 5, 3, 3, 5, 7, 8],
        [7, 6, 4, 2, 2, 4, 5],
        [8, 7, 5, 3, 3, 5, 6],
        [9, 8, 6, 4, 2, 2, 3],
        [10, 9, 7, 5, 3, 3, 4] // D-Bottom to Hallway
    ];

    // Cost to get from room to room 0 -0 0 -1, .... 7 - 7
    // Symmetric matrix
    let room_cost = [
        [0, 1, 4, 5, 6, 7, 8, 9],
        [1, 0, 5, 6, 7, 8, 9, 10],
        [4, 5, 0, 1, 4, 5, 6, 7],
        [5, 6, 1, 0, 5, 6, 7, 8],
        [6, 7, 4, 5, 0, 1, 4, 5],
        [7, 8, 5, 6, 1, 0, 5, 6],
        [8, 9, 6, 7, 4, 5, 0, 1],
        [9, 10, 7, 8, 5, 6, 1, 0]
    ];

    let is_room_right = |pod: &Option<Pod>, room_no: usize| -> bool {
        if pod.is_none() {
            return false;
        }
        match pod.unwrap() {
            Pod::A1 | Pod::A2 => room_no == 0 || room_no == 1,
            Pod::B1 | Pod::B2 => room_no == 2 || room_no == 3,
            Pod::C1 | Pod::C2 => room_no == 4 || room_no == 5,
            Pod::D1 | Pod::D2 => room_no == 6 || room_no == 7
        }
    };
    let reached_end = |rooms: &[Option<Pod>; 8]| -> bool {
        rooms.iter().enumerate().all(|(i, val)| is_room_right(val, i))
    };
    let right_room = |pod: &Pod| -> [usize; 2] {
        match pod {
            Pod::A1 | Pod::A2 => [0, 1],
            Pod::B1 | Pod::B2 => [2, 3],
            Pod::C1 | Pod::C2 => [4, 5],
            Pod::D1 | Pod::D2 => [6, 7],
        }
    };
    let right_bottom_room = |pod: &Pod, room_no| -> bool {
        match pod {
            Pod::A1 | Pod::A2 => room_no == 1,
            Pod::B1 | Pod::B2 => room_no == 3,
            Pod::C1 | Pod::C2 => room_no == 5,
            Pod::D1 | Pod::D2 => room_no == 7,
        }
    };
    let get_cost = |pod: &Pod| -> i64 {
        match pod {
            Pod::A1 | Pod::A2 => 1,
            Pod::B1 | Pod::B2 => 10,
            Pod::C1 | Pod::C2 => 100,
            Pod::D1 | Pod::D2 => 1000,
        }
    };

    let mut state_space: HashMap<State, i64> = HashMap::new();
    let mut hallway = [None; 7];
    let start_state = State { rooms: start, hallway: hallway };
    state_space.insert(start_state.clone(), 0);
    let mut heap = BinaryHeap::new();
    heap.push((0, start_state.clone()));
    let mut min_val = i64::MAX;

    while !heap.is_empty() {
        let (cost, state) = heap.pop().unwrap();
        let cost = -1 * cost; // Since this is a max-heap we need to negate
        if reached_end(&state.rooms) {
            println!("Reached end-config {:?}", state);
            println!("Cost: {}", cost);
            min_val = min(min_val, cost);
            continue;
        }
        // Consider all pods in rooms and see if they can be moved to the right room if not already in one.
        // Generate new state and check if new_state is seen and if so, if the cost is less than.
        for (i, room) in state.rooms.iter().enumerate() {
            if room.is_none() {
                continue;
            }
            if right_bottom_room(&room.unwrap(), i) {
                continue;
            }
            for room_no in right_room(&room.unwrap()) {
                // Right Room is not empty
                if !state.rooms[room_no].is_none() {
                    continue;
                }
                let mut new_rooms = state.rooms.clone();
                new_rooms[i] = None;
                new_rooms[room_no] = *room;
                let num_steps = room_cost[i][room_no];
                let add_cost = cost + num_steps * get_cost(&room.unwrap());

                let new_state = State { rooms: new_rooms, hallway: state.hallway.clone() };
                let ch = state_space.entry(new_state).or_insert(i64::MAX);
                if *ch > add_cost {
                    *ch = add_cost;
                    heap.push((-1 * add_cost, new_state));
                }
            }
        }

        // Consider all pods in hallways and see if some can be moved to rooms immediately 
        for (hall_no, hall) in state.hallway.iter().enumerate() {
            if hall.is_none() {
                continue;
            }
            for room_no in right_room(&hall.unwrap()) {
                if !state.rooms[room_no].is_none() {
                    continue;
                }
                let mut new_rooms = state.rooms.clone();
                new_rooms[room_no] = *hall;
                let mut new_hallway = state.hallway.clone();
                new_hallway[hall_no] = None;
                let num_steps = hall_cost[room_no][hall_no];
                let add_cost = cost + num_steps * get_cost(&hall.unwrap());
                let new_state = State { rooms: new_rooms, hallway: new_hallway };

                let ch = state_space.entry(new_state).or_insert(i64::MAX);
                if *ch > add_cost {
                    *ch = add_cost;
                    heap.push((-1 * add_cost, new_state));
                }
            }
        }
        
        // Consider all room pods and see if some can be moved to the hallway
        for (room_no, room) in state.rooms.iter().enumerate() {
            if room.is_none() || right_bottom_room(&room.unwrap(), room_no) {
                continue;
            }
            for (hall_no, hall) in state.hallway.iter().enumerate() {
                if !hall.is_none() {
                    continue;
                }
                let mut new_rooms = state.rooms.clone();
                new_rooms[room_no] = None;
                let mut new_hallway = state.hallway.clone();
                new_hallway[hall_no] = *room;
                let num_steps = hall_cost[room_no][hall_no];
                let add_cost = cost + num_steps * get_cost(&room.unwrap());

                let new_state = State { rooms: new_rooms, hallway: new_hallway };
                let ch = state_space.entry(new_state).or_insert(i64::MAX);
                if *ch > add_cost {
                    *ch = add_cost;
                    heap.push((-1 * add_cost, new_state));
                }
            }
        }
    }
    min_val
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        println!("Part1: {}", dijkstra([Some(Pod::B1), Some(Pod::A1), Some(Pod::C1), Some(Pod::D1), Some(Pod::B2), Some(Pod::C2), Some(Pod::D2), Some(Pod::A2)]));
    }
}
