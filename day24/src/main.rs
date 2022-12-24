use std::{collections::BinaryHeap, io::Error};
use aochelpers::{Coordinate,ScoredItem};
use std::collections::{HashMap, HashSet};

fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("./day24/input.txt")?;
    let mountain = parse_data(&data);
    let mut stage_2_mountain = mountain.clone();
    let mut stage_3_mountain = mountain.clone();
    let mut snow_cache = HashMap::new();
    let stage_2_start = get_off_the_mountain(mountain, false, &mut snow_cache); 
    println!("Part 1: {}", stage_2_start);
    stage_2_mountain.start_time = stage_2_start;
    let stage_3_start = get_off_the_mountain(stage_2_mountain, true, &mut snow_cache); 
    stage_3_mountain.start_time = stage_3_start;
    println!("Part 2: {}", get_off_the_mountain(stage_3_mountain, false, &mut snow_cache)); 
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct GameState {
    start: Coordinate<i32>,
    winds: Vec<Wind>,
    target: Coordinate<i32>,
    start_time: i32
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_off_the_mountain(mountain: GameState, swap_ends: bool, cache: &mut HashMap<(Coordinate<i32>, i32),bool>) -> i32{

    let mut states = BinaryHeap::new();
    let starting_state = if ! swap_ends { 
        ScoredItem{ cost: mountain.start_time, item: mountain.start}
    } else {
        ScoredItem{ cost: mountain.start_time, item: mountain.target}
    };

    let target = if !swap_ends {
        mountain.target
    } else {
        mountain.start
    };

    states.push(starting_state);
    let mut seen = HashSet::new();
    let cycle_time = lcm(mountain.target.x.max(mountain.start.x), (mountain.start.y -1).max(mountain.target.y -1));
    let max_y = mountain.start.y.max(mountain.target.y);
    while let Some(state) = states.pop() {
        if seen.contains(&(state.item, state.cost % cycle_time)) {
            continue;
        }

        seen.insert((state.item, state.cost % cycle_time));
        
        if state.item == target {
            return state.cost
        }
        let mut candidates = state.item.neighbours();
        candidates.push(state.item);
        for candidate in candidates {
            if !((candidate.x == 0 || candidate.y == 0 && candidate != Coordinate{x:1, y:0}) ||
            (candidate.y == mountain.target.y && candidate.x != mountain.target.x) ||
            candidate.x == mountain.target.x +1 ||
            candidate.y < 0 || candidate.y > max_y) {
                let winds = cache.entry((candidate, (state.cost +1) % cycle_time)).or_insert_with(|| is_snowy(candidate, &mountain, (state.cost +1) % cycle_time));
                if ! *winds {
                    states.push(ScoredItem{cost: state.cost +1, item: candidate})
                }
            }
        }
    }
    i32::MAX
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
    GameState { start: Coordinate {x: 1, y: 0}, winds, target: Coordinate {x: max_x as i32, y: max_y as i32}, start_time: 0 }
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
        let mountain = parse_data(DATA);
        let mut stage_2_mountain = mountain.clone();
        let mut stage_3_mountain = mountain.clone();
        let mut snow_cache = HashMap::new();
        let stage_2_start = get_off_the_mountain(mountain, false, &mut snow_cache); 
        assert_eq!(stage_2_start, 18);
        stage_2_mountain.start_time = stage_2_start;
        let stage_3_start = get_off_the_mountain(stage_2_mountain, true, &mut snow_cache); 
        assert_eq!(stage_3_start, 41);
        stage_3_mountain.start_time = stage_3_start;
        assert_eq!(get_off_the_mountain(stage_3_mountain, false, &mut snow_cache), 54); 
    }
}