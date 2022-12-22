use std::error::Error;
use hashbrown::HashMap;
use aochelpers::{Coordinate, parse_number_grid};

fn main() -> Result<(), Box<dyn Error>> {
    let data = std::fs::read_to_string("./day08/input.txt")?;
    let grid = parse_number_grid::<usize>(&data);
    let bounds = Coordinate{x: grid.keys().map(|c| c.x).max().unwrap(), y: grid.keys().map(|c| c.y).max().unwrap()};
    println!("Part 1: {}", part1(&grid, &bounds));
    println!("Part 2: {}", part2(&grid, &bounds));

    Ok(())
}

fn part1(grid: &HashMap<Coordinate<usize>, usize>, bounds: &Coordinate<usize>) -> usize {
    grid.keys().filter(|s| is_visible(s, grid, bounds)).count()
}

fn part2(grid: &HashMap<Coordinate<usize>, usize>, bounds: &Coordinate<usize>) -> usize {
    grid.keys().map(|c| scenic_score(c, grid, bounds),).max().unwrap()
}

fn is_visible(loc: &Coordinate<usize>, grid:&HashMap<Coordinate<usize>, usize>, bounds: &Coordinate<usize>) -> bool{
    let current_height = grid.get(loc).unwrap();
    (0..loc.x).all(|x| grid.get(&Coordinate{x, y: loc.y}).unwrap() < current_height) ||
    (0..loc.y).all(|y| grid.get(&Coordinate{x: loc.x, y}).unwrap() < current_height) ||
    (loc.x+1..=bounds.x).all(|x| grid.get(&Coordinate{x, y: loc.y}).unwrap() < current_height) ||
    (loc.y+1..=bounds.y).all(|y| grid.get(&Coordinate{x: loc.x, y }).unwrap() < current_height) 
}

fn scenic_score(loc: &Coordinate<usize>, grid:&HashMap<Coordinate<usize>, usize>, bounds: &Coordinate<usize>) -> usize{

    let current_height = grid.get(loc).unwrap();
    let left  = if let Some(v) = (0..loc.x).rev().position(|x| grid.get(&Coordinate{x, y: loc.y}).unwrap() >= current_height) {v+1} else { loc.x};
    let right = if let Some(v) = (loc.x+1..=bounds.x).position(|x| grid.get(&Coordinate{x, y: loc.y}).unwrap() >= current_height) {v+1} else { bounds.x - loc.x};
    let up = if let Some(v) = (0..loc.y).rev().position(|y| grid.get(&Coordinate{x: loc.x, y}).unwrap() >= current_height) {v+1} else {loc.y};
    let down = if let Some(v) = (loc.y+1..=bounds.y).position(|y| grid.get(&Coordinate{x: loc.x, y}).unwrap() >= current_height) {v+1} else {bounds.y - loc.y};
    left * right * up * down
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "30373
25512
65332
33549
35390
";

    #[test]
    fn test_part_1 (){
     let grid = parse_number_grid::<usize>(DATA);
     let bounds =     Coordinate{x: grid.keys().map(|c| c.x).max().unwrap(), y: grid.keys().map(|c| c.y).max().unwrap()};

     assert!(is_visible(&Coordinate{x:0,y:0}, &grid, &bounds));
     assert!(is_visible(&Coordinate{x:4,y:4}, &grid, &bounds));
     assert!(is_visible(&Coordinate{x:0,y:4}, &grid, &bounds));
     assert!(is_visible(&Coordinate{x:4,y:0}, &grid, &bounds));
     assert!(is_visible(&Coordinate{x:1,y:1}, &grid, &bounds));
     assert!(!is_visible(&Coordinate{x:2,y:2}, &grid, &bounds));
     assert_eq!(part1(&grid, &bounds), 21);
    }

    #[test]
    fn test_part_2() {
        let grid = parse_number_grid::<usize>(DATA);
        let bounds = Coordinate{x: grid.keys().map(|c| c.x).max().unwrap(), y: grid.keys().map(|c| c.y).max().unwrap()};
        assert_eq!(scenic_score(&Coordinate { x: 2, y: 1 }, &grid, &bounds), 4);
        assert_eq!(scenic_score(&Coordinate { x: 2, y: 3 }, &grid, &bounds), 8);
        assert_eq!(part2(&grid, &bounds), 8);

    }

}