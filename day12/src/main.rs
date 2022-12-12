use std::{io::Error, collections::{HashMap, BinaryHeap, HashSet}};
use aochelpers::{self, Coordinate, ScoredItem};

#[derive(Debug, Clone)]
struct Mountain{
    start: Coordinate<i32>,
    end: Option<Coordinate<i32>>,
    ground_map: HashMap<Coordinate<i32>,usize>
}

fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("./day12/input.txt")?;
    let mountain = parse_mountain(&data);
    println!("Part 1: {}", solution(&mountain).unwrap());
    println!("Part 2: {}", part2(&mountain).unwrap());
    Ok(())
}

fn solution(mountain: &Mountain) -> Option<usize> {
    let mut next_steps = BinaryHeap::from([ScoredItem{cost: 0_usize, item: mountain.start}]);
    let mut seen = HashSet::new();

    while !next_steps.is_empty() {
        let step = next_steps.pop().unwrap();
        if seen.contains(&step.item) {
            continue;
        }
        /*
          If we've hit
            - the prescribed ending point 
            - a point at the highest level and we don't have a specific endpoint
          We've found the finish
        */
        if Some(step.item) == mountain.end || mountain.end.is_none() && mountain.ground_map.get(&step.item).unwrap() == &26 {
            return Some(step.cost);
        }
        seen.insert(step.item);
        for neighbour in step.item.neighbours() {
            if *mountain.ground_map.get(&neighbour).unwrap_or(&usize::MAX) <= mountain.ground_map.get(&step.item).unwrap() +1 {
                next_steps.push(ScoredItem { cost: step.cost +1, item: neighbour});
            }
        }
    }
    None
}


fn part2(mountain: &Mountain) -> Option<usize> {
    /* We want the lowest point that has the shortest path to the target. 
       Flip the map so that "E" and "z" are the lowest points,
       so we can find the closest point at level "a".
       Set start point to be original and, and endpoint as "don't care". */

    let ground_map = mountain.ground_map.iter()
        .map(|(a,b)| (*a, 27-b))
        .collect::<HashMap<_,_>>();
    solution(&Mountain{start: mountain.end.unwrap(), end: None, ground_map})
}

fn parse_mountain(data: &str) -> Mountain{
    let mut map = HashMap::new();
    let mut start = Coordinate {x:0, y:0};
    let mut end = Coordinate {x:0,y:0};
    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start.x = x as i32;
                start.y = y as i32;
                map.insert(Coordinate{x: x as i32 ,y: y as i32}, 'a' as usize - 96);
            } else if c == 'E' {
                end.x = x as i32;
                end.y = y as i32;
                map.insert(Coordinate{x: x as i32 ,y: y as i32}, 'z' as usize - 96 );
            } else {
                map.insert(Coordinate{x: x as i32 ,y: y as i32}, c as usize - 96 );
            }
        }
    }
    Mountain {start, end: Some(end), ground_map: map}
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1() {
        // let input = DATA.split("\n").collect::<Vec<_>>();
        assert_eq!(solution(&parse_mountain(DATA)), Some(31))
    }

    #[test]
    fn test_part2() {
        
        assert_eq!(part2(&parse_mountain(DATA)), Some(29))

    }
}