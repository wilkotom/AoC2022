use std::io::Error;
use aochelpers::Coordinate;
use hashbrown::HashSet;


fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("./day14/input.txt")?;
    let area = build_map(&data);
    let part1 = solution(area.clone(), false);
    let part2 = solution(area, true);
    println!("Part 1: {:?}\nPart 2: {:?}", part1, part2);
    Ok(())
}

fn solution(mut area: HashSet<Coordinate<i32>>, part2: bool) -> i32 {
    let mut grain_count = 0;
    let max_y = area.iter().map(|c| c.y).max().unwrap();
    while let Some(grain_loc) = simulate_grain(&area, max_y, part2) {
        area.insert(grain_loc);
        grain_count += 1;
    }
    grain_count
}

fn simulate_grain(area: &HashSet<Coordinate<i32>>, max_y: i32, part2: bool) -> Option<Coordinate<i32>> {
    let mut grain = Coordinate{x: 500, y: 0};
    let floor = max_y + 2;
    if area.contains(&Coordinate { x: 500, y: 0 }) {
        return None;
    }
    while !area.contains(&Coordinate { x: grain.x, y: grain.y+1 }) || 
            !area.contains(&Coordinate { x: grain.x-1, y: grain.y+1 }) ||
            !area.contains(&Coordinate { x: grain.x+1, y: grain.y+1 })
            && grain.y < floor {
        if grain.y == floor-1 {
            if part2 {
                break;
            } else {
                return None
            }
        } else if !area.contains(&Coordinate { x: grain.x, y: grain.y+1 }) {
            grain.y += 1;
        } else if !area.contains(&Coordinate { x: grain.x-1, y: grain.y+1 }) {
            grain.x -= 1;
            grain.y +=1;
        } else {
            grain.x += 1;
            grain.y +=1;
        }
    }
    Some(grain)
    
}

fn build_map(data: &str) -> HashSet<Coordinate<i32>> {
    let mut area = HashSet::new();
    for line in data.lines() {
        let mut points = line.split(" -> ")
            .map(|c| c.split(','))
            .map(|mut s| Coordinate::<i32>{
                x: s.next().unwrap().parse::<i32>().unwrap(), 
                y: s.next().unwrap().parse::<i32>().unwrap()});
        let mut start;
        let mut end = points.next().unwrap();
        for next_point in points {
            start = end;
            end = next_point;
            for x in start.x.min(end.x)..=start.x.max(end.x) {
                for y in start.y.min(end.y)..=start.y.max(end.y)  {
                    area.insert(Coordinate {x, y});
                }
            } 
        }
    }
    area
}


#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part1() {
        let area = build_map(DATA);
        assert_eq!(solution(area, false), 24)
    }

    #[test]
    fn test_part2() {
        let area = build_map(DATA);
        assert_eq!(solution(area, true), 93)
    }
}