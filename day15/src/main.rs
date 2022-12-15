use std::{ io::Error, collections::{HashMap, HashSet}};
use aochelpers::Coordinate;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("Sensor at x={sensor_x}, y={sensor_y}: closest beacon is at x={beacon_x}, y={beacon_y}")]
struct SensorBeaconPair {
    sensor_x: i128,
    sensor_y: i128,
    beacon_x: i128,
    beacon_y: i128
}

#[derive(Debug,Copy,Clone)]
struct Beacon {
    location: Coordinate<i128>,
    radius: i128
}

fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("./day15/input.txt")?;
    let part1 = solution(&data, 2000000);
    let part2 = solution2(&data, 4000000);
    println!("Part 1: {:?}\nPart 2: {:?}", part1, part2.unwrap());
    Ok(())
}

fn solution(data: &str, y: i128) -> i128 {
    let sensors = parse_lines(data);
    // Search space: minimum X of all sensors, minus the maximum radius of all beacons
    // to maximum X of all sensors, plus the maximum radius of all beacons
    let max_radius = sensors.values().map(|s| s.radius).max().unwrap();
    let start_x = sensors.keys().map(|c| c.x).min().unwrap() - max_radius;
    let end_x = sensors.keys().map(|c| c.x).max().unwrap() + max_radius;

    let beacons = sensors.values().map(|b| b.location).collect::<HashSet<_>>();
    let mut total = 0;
    for x in start_x..=end_x {
        let candidate = Coordinate{x,y};
        if beacons.contains(&candidate) || sensors.contains_key(&candidate) {
            continue;
        }
        total += sensors.iter().any(|(s,b)| candidate.manhattan_distance(s) < b.radius)as i128;
    }
    total
}


fn solution2(data: &str, radius: i128) -> Option<i128> {

    let sensors = parse_lines(data);
    for (i, (sensor, beacon)) in sensors.iter().enumerate() {
        // Consider only sensors which have at least 1 other sensors exactly r1 + r2 +2 distance away
        for (other, target) in sensors.iter().skip(i+1) {
            if sensor != other && sensor.manhattan_distance(other) == beacon.radius + target.radius +2 {
                let circle = manhattan_circle(sensor, beacon.radius+1);
                for point in circle.iter().filter(|p| p.x >=0 && p.x <= radius && p.y >=0 && p.y <= radius) {
                    if sensors.iter().all(|(s,b)| point.manhattan_distance(s) > b.radius) {
                        return Some(point.x * 4000000 + point.y);
                    }
                }
            }
        }
    }
    Some(0)
}


fn manhattan_circle(point: &Coordinate<i128>, radius: i128) -> Vec<Coordinate<i128>> {

    let mut circle = Vec::new();
    for i in 0..radius {
        circle.push(Coordinate{x: point.x - radius + i, y: point.y +i});
        circle.push(Coordinate{x: point.x+i, y: point.y + radius -i});
        circle.push(Coordinate{x: point.x +radius - i, y: point.y - i});
        circle.push(Coordinate{x: point.x - i, y: point.y - (radius -i)});
    }
    circle
}

fn parse_lines(data: &str) -> HashMap<Coordinate<i128>, Beacon> {
    let mut sensors = HashMap::new();
    for line in data.lines() {
        let sbp = line.parse::<SensorBeaconPair>().unwrap();
        let sensor_loc = Coordinate{x: sbp.sensor_x, y: sbp.sensor_y};
        let beacon_loc = Coordinate{x: sbp.beacon_x, y: sbp.beacon_y};
        let beacon_dist = Beacon { location:beacon_loc, radius: sensor_loc.manhattan_distance(&beacon_loc) };
        sensors.insert(sensor_loc, beacon_dist);
    }
    sensors
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part1() {
        solution(DATA, 10);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solution2(DATA, 20), Some(56000011));
    }
}
