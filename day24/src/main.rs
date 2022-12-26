use std::{collections::BinaryHeap, io::Error};
use aochelpers::{Coordinate,ScoredItem};
use hashbrown::{HashMap, HashSet};
use std::cell::RefCell;
use std::rc::Rc;

fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("./day24/input.txt")?;
    let mut mountain = parse_data(&data);
    get_off_the_mountain(&mut mountain);
    println!("Part 1: {}", mountain.current_time);
    get_off_the_mountain(&mut mountain);
    get_off_the_mountain(&mut mountain);
    println!("Part 2: {}", mountain.current_time);

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Wind {
    starting_point: Coordinate<i32>,
    direction: Direction
}

type BlizzardCache = HashMap<(Coordinate<i32>, i32),bool>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct GameState {
    start: Coordinate<i32>,
    winds: Vec<Wind>,
    target: Coordinate<i32>,
    current_time: i32,
    reverse_path: bool,
    blizzard_cache: Rc<RefCell<BlizzardCache>>
}


fn get_off_the_mountain(mountain: &mut GameState) -> Option<i32>{
    let mut states = BinaryHeap::new();
    let starting_state = if ! mountain.reverse_path { 
        ScoredItem{ cost: mountain.current_time, item: (mountain.start, mountain.current_time)}
    } else {
        ScoredItem{ cost: mountain.current_time, item: (mountain.target, mountain.current_time)}
    };
    let target = if !mountain.reverse_path {
        mountain.target
    } else {
        mountain.start
    };

    states.push(starting_state);
    let mut seen = HashSet::new();
    let cycle_time = lcm(mountain.target.x.max(mountain.start.x), (mountain.start.y -1).max(mountain.target.y -1));
    let max_y = mountain.target.y;
    while let Some(state) = states.pop() {
        let (location, time) = state.item;
        if seen.contains(&(location, time % cycle_time)) {
            continue;
        }
        seen.insert((location, time % cycle_time));
        if location == target {
            mountain.current_time = time;
            mountain.reverse_path = !mountain.reverse_path;
            return Some(time)
        }
        let mut candidates = location.neighbours();
        candidates.push(location);
        for candidate in candidates {
            if !((candidate.x == 0 || candidate.y == 0 && candidate != Coordinate{x:1, y:0}) ||
            (candidate.y == mountain.target.y && candidate.x != mountain.target.x) ||
            candidate.x == mountain.target.x +1 ||
            candidate.y < 0 || candidate.y > max_y) {
                let mut cache = mountain.blizzard_cache.borrow_mut();
                let winds = cache.entry((candidate, (time +1) % cycle_time)).or_insert_with(|| is_snowy(candidate, &mountain, (time +1) % cycle_time));
                if ! *winds {
                    // Weight strongly in favour of distance to target; blizzards will likely significantly increase number of actual steps needed
                    let heuristic = candidate.manhattan_distance(&target) *2 + time;
                    states.push(ScoredItem{cost: heuristic, item: (candidate, time +1)});
                }
            }
        }
    }
    None
}


fn parse_data(data: &str) -> GameState {
    let mut max_y = 0;
    let mut max_x = 0;
    let mut winds = Vec::new();
    for (y,line) in data.lines().enumerate() {
        for (x,c) in line.chars().enumerate() {
            match c {
                '#' => {},
                '.' => {
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                },
                '^' => {winds.push(Wind{starting_point: Coordinate { x: x as i32, y: y as i32}, direction: Direction::North})},
                'v' => {winds.push(Wind{starting_point: Coordinate { x: x as i32, y: y as i32}, direction: Direction::South})},
                '>' => {winds.push(Wind{starting_point: Coordinate { x: x as i32, y: y as i32}, direction: Direction::East})},
                '<' => {winds.push(Wind{starting_point: Coordinate { x: x as i32, y: y as i32}, direction: Direction::West})},
                _ => unimplemented!("Can't parse {}", c)
            };
        }
    }
    GameState { start: Coordinate {x: 1, y: 0}, 
                winds, 
                target: Coordinate {x: max_x as i32, y: max_y as i32}, 
                current_time: 0, 
                reverse_path: false, 
                blizzard_cache:Rc::new(RefCell::new(HashMap::new()))}
}

fn is_snowy(location: Coordinate<i32>, state: &GameState, turn: i32) ->bool{
    let bottom_left = state.start.max(state.target);
    for wind in state.winds.iter() {
        match wind.direction {
            Direction::North =>  {
                if wind.starting_point.x == location.x && (wind.starting_point.y -1 - turn).rem_euclid(bottom_left.y -1 ) +1 == location.y { 
                   return true;
                }
            },
            Direction::South => {
                if wind.starting_point.x == location.x && (wind.starting_point.y -1 + turn) % (bottom_left.y -1) +1 == location.y {
                    return true;
                }
            }
            Direction::East => {    
                if wind.starting_point.y == location.y && (wind.starting_point.x -1 + turn) % (bottom_left.x ) +1 == location.x {
                    return true;
                }
            },
            Direction::West => {
                if wind.starting_point.y == location.y && (wind.starting_point.x -1 - turn).rem_euclid(bottom_left.x ) +1 == location.x { 
                    return true;
                }
            },
        }
    }
    false
}

fn lcm(first: i32, second: i32) -> i32 {
    first * second / gcd(first, second)
}

fn gcd(first: i32, second: i32) -> i32 {
    let mut max = first;
    let mut min = second;
    if min > max {
        (min, max) = (max,min)
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_day1() {
        let mut mountain = parse_data(DATA);
        get_off_the_mountain(&mut mountain);
        assert_eq!(mountain.current_time, 18);
        get_off_the_mountain(&mut mountain);
        assert_eq!(mountain.current_time, 41);
        get_off_the_mountain(&mut mountain);
        assert_eq!(mountain.current_time, 54);
    }
}