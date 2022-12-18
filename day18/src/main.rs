use std::{collections::{HashSet, VecDeque}, io::Error};
use aochelpers::{Coordinate3d, Cuboid};

fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("./day18/input.txt")?;
    let (part1, part2) = solution(&data);
    println!("Part 1: {:?}\nPart 2: {:?}", part1, part2);
    Ok(())
}

fn solution(data: &str) -> (usize,usize) {

    let mut cubes = HashSet::new();
    for line in data.lines() {
        let mut numbers = line.split(',').map(|n| n.parse::<i32>().unwrap());
        let cube =Coordinate3d{x: numbers.next().unwrap(), y: numbers.next().unwrap(), z: numbers.next().unwrap()};
        cubes.insert(cube);
    }

    let visible_faces =  cubes.len() * 6 - cubes.iter().map(|c| c.neighbours().iter().filter(|c| cubes.contains(c)).count()).sum::<usize>();

    let part1 = visible_faces;

    let mut cloud = cubes.clone();

    let bounding_box = Cuboid{
        top_left_back: Coordinate3d { x: cubes.iter().map(|c| c.x).min().unwrap(),  
                                      y: cubes.iter().map(|c| c.y).min().unwrap(), 
                                      z: cubes.iter().map(|c| c.z).min().unwrap(), },
        bottom_right_front: Coordinate3d { x: cubes.iter().map(|c| c.x).max().unwrap(), 
                                           y: cubes.iter().map(|c| c.y).max().unwrap(), 
                                           z: cubes.iter().map(|c| c.z).max().unwrap(), }};
    
    let starting_point = bounding_box.top_left_back;

    let mut unconsidered = VecDeque::new();
    unconsidered.push_back(starting_point);

    while let Some(point) = unconsidered.pop_front() {
        if bounding_box.contains(&point){
            for neighbour in point.neighbours() {
                if !cloud.contains(&neighbour) && !unconsidered.contains(&neighbour){
                    unconsidered.push_back(neighbour);
                }
            }
            cloud.insert(point);
        }
    }

    for x in bounding_box.top_left_back.x..bounding_box.bottom_right_front.x {
        for y in bounding_box.top_left_back.y..bounding_box.bottom_right_front.y {
            for z in  bounding_box.top_left_back.z..bounding_box.bottom_right_front.z {
                if !cloud.contains(&Coordinate3d{x,y,z}) {
                    cubes.insert(Coordinate3d{x,y,z});
                }
            }
        }
    }
    let part2_faces =  cubes.len() * 6 - cubes.iter().map(|c| c.neighbours().iter().filter(|c| cubes.contains(c)).count()).sum::<usize>();

    // let mut part2_faces = cubes.len() *6;
    // for cube in cubes.iter() {
    //     for neighbour in cube.neighbours() {
    //         if cubes.contains(&neighbour) {
    //             part2_faces -=1;
    //         }
    //     }
    // }
    println!("{} {}", part1, part2_faces);
    (part1,part2_faces)
}


#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_day1() {
        let (part1, part2) = solution(DATA);
        assert_eq!(part1, 64);
        assert_eq!(part2, 58);
    }
}