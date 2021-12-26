use std::collections::{HashMap, BinaryHeap};
use std::cmp::{Ordering, min, max};

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
enum Pod {
    A,
    B,
    C,
    D
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
            Pod::A => room_no == 0 || room_no == 1,
            Pod::B => room_no == 2 || room_no == 3,
            Pod::C => room_no == 4 || room_no == 5,
            Pod::D => room_no == 6 || room_no == 7
        }
    };
    let reached_end = |rooms: &[Option<Pod>; 8]| -> bool {
        rooms.iter().enumerate().all(|(i, val)| is_room_right(val, i))
    };
    let right_room = |pod: &Pod| -> [usize; 2] {
        match pod {
            Pod::A => [0, 1],
            Pod::B => [2, 3],
            Pod::C => [4, 5],
            Pod::D => [6, 7],
        }
    };
    let right_bottom_room = |pod: &Pod, room_no| -> bool {
        match pod {
            Pod::A => room_no == 1,
            Pod::B => room_no == 3,
            Pod::C => room_no == 5,
            Pod::D => room_no == 7,
        }
    };
    let get_cost = |pod: &Pod| -> i64 {
        match pod {
            Pod::A => 1,
            Pod::B => 10,
            Pod::C => 100,
            Pod::D => 1000,
        }
    };
    // Check if path between room and hall is free, including the room or hall
    // If to_room flag is set, check if the room is empty
    // If to_hall flag is set, check if hall is empty
    // Always go from room to hall even if the orig operation is hall->room. 
    // The booleans ensure that the destination is free and the rest of the path is the same
    fn can_move_room_hallway(room_no: usize, hall_no: usize, state: &State, to_room: bool, to_hall: bool) -> bool {
        let mut left;
        let mut right;
        if to_room && !state.rooms[room_no].is_none() || to_hall && !state.hallway[hall_no].is_none() {
            return false;
        }
        if room_no == 0 || room_no == 1 {
            left = vec![0, 1];
            right = vec![2, 3, 4, 5, 6];
        }
        else if room_no == 2 || room_no == 3 {
            left = vec![0, 1, 2];
            right = vec![3, 4, 5, 6];
        }
        else if room_no == 4 || room_no == 5 {
            left = vec![0, 1, 2, 3];
            right = vec![4, 5, 6];
        }
        else {
            left = vec![0, 1, 2, 3, 4];
            right = vec![5, 6];
        }
        // If bottom room, check that move from top-room to hall is possible and top room is empty
        if room_no == 1 || room_no == 3 || room_no == 5 || room_no == 7 {
            return can_move_room_hallway(room_no - 1, hall_no, state, to_room, to_hall) && state.rooms[room_no - 1].is_none();
        }
        left.reverse();
        if left.contains(&hall_no) {
            for val in left {
                if val == hall_no {
                    break;
                }
                if !state.hallway[val].is_none() {
                    return false;
                }
            }
        }
        if right.contains(&hall_no) {
            for val in right {
                if val == hall_no {
                    break;
                }
                if !state.hallway[val].is_none() {
                    return false;
                }
            }
        }
        return true;
    };

    // Check if room-room move is possible
    // First check if destination room is free
    // Then check if room->temp-hall and temp-hall->room is possible
    let can_move_room_room = |room_no1: usize, room_no2: usize, state: &State| -> bool {
        let hallway = [2, 3, 4];
        // Check if destination room is empty
        if !state.rooms[room_no2].is_none() {
            return false;
        }
        let (small_room, big_room) = (min(room_no1, room_no2), max(room_no1, room_no2));
        let mut temp_hall = 0;
        if small_room <= 1 {
            if big_room <= 3 {
                temp_hall = 2;
            }
            else if big_room <= 5 {
                temp_hall = 3;
            }
            else {
                temp_hall = 4;
            }
        }
        else if small_room <= 3 {
            if big_room <= 5 {
                temp_hall = 3;
            }
            else {
                temp_hall = 4;
            }
        }
        else {
            temp_hall = 4;
        }
        return can_move_room_hallway(small_room, temp_hall, state, false, true) && can_move_room_hallway(big_room, temp_hall, state, false, true);
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
                if !can_move_room_room(i, room_no, &state) {
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
                if !can_move_room_hallway(room_no, hall_no, &state, true, false) {
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
                if !can_move_room_hallway(room_no, hall_no, &state, false, true) {
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
        println!("Part1: {}", dijkstra([Some(Pod::B), Some(Pod::A), Some(Pod::C), Some(Pod::D), Some(Pod::B), Some(Pod::C), Some(Pod::D), Some(Pod::A)]));
    }

    #[test]
    fn actual() {
        println!("Part1: {}", dijkstra([Some(Pod::D), Some(Pod::C), Some(Pod::B), Some(Pod::A), Some(Pod::D), Some(Pod::A), Some(Pod::B), Some(Pod::C)]));
    }
}
