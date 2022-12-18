use std::{ io::Error, str::FromStr, collections::{HashMap, VecDeque}};

#[derive(Eq, PartialEq, Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: i64,
    tunnels: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseValveError;

#[derive(Eq, PartialEq, Debug, Clone)]
struct SeenState {
    time: i64,
    flowed: i64,
}

impl FromStr for Valve {
    type Err = ParseValveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split(' ').collect::<Vec<_>>();
        let name = words[1].to_string();
        let flow_rate = words[4].trim_start_matches("rate=").trim_end_matches(';').parse::<i64>().unwrap();
        let mut tunnels = vec![];
        for word in words[9..].iter() {
            tunnels.push(word.trim_end_matches(',').to_string());
        }
        Ok(Self {
            name, 
            flow_rate,
            tunnels,
        })
    }
}

fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("./day16/input.txt")?;

    let valves = parse_data(&data);
    let distances = distances(&valves);
    let working_valves = valves.values()
        .filter(|v| v.flow_rate > 0)
        .map(|v| v.name.clone())
        .collect::<Vec<_>>();
    println!("Part 1: {:?}", part1(&valves, &distances, &working_valves));
    println!("Part 1: {:?}", part2(&valves, &distances, &working_valves));

    Ok(())
}

fn part1(valves: &HashMap<String,Valve>,distances: &HashMap<String,HashMap<String,i64>>, working_valves: &[String]) -> i64 {

    let mut unchecked = working_valves.iter().map(|v| vec![v.to_owned()]).collect::<VecDeque<_>>();

    let mut best = 0;
    while let Some(combo) = unchecked.pop_front() {
        if let Some(score) = ordering_score(&combo, distances, valves, 30) {
            best = best.max(score);
            for next_valve in working_valves.iter().filter(|v| !combo.contains(v)) {
                let mut next_combo = combo.clone();
                next_combo.push(next_valve.to_string());
                unchecked.push_back(next_combo);
            }
        }
    }
    best
}


fn part2(valves: &HashMap<String,Valve>,distances: &HashMap<String,HashMap<String,i64>>, working_valves: &[String]) -> i64 {

    let mut unchecked = working_valves.iter().map(|v| vec![v.to_owned()]).collect::<VecDeque<_>>();

    let mut path_scores = Vec::new();
    while let Some(combo) = unchecked.pop_front() {
        if let Some(score) = ordering_score(&combo, distances, valves, 26) {
            path_scores.push((combo.clone(), score));
            for next_valve in working_valves.iter().filter(|v| !combo.contains(v)) {
                let mut next_combo = combo.clone();
                next_combo.push(next_valve.to_string());
                unchecked.push_back(next_combo);
            }
        }
    }
    let mut best = 0;
    for (i, (left_path, left_path_score)) in path_scores.iter().enumerate().filter(|(_,(p,_))| p.len() >= working_valves.len() / 3) {
        for (right_path, right_path_score) in path_scores[i+1..].iter().filter(|(p,_)| p.len() >= working_valves.len() / 3){
            if right_path.iter().all(|v| !left_path.contains(v)) {
                let score = left_path_score + right_path_score;
                best = best.max(score);
            }
        }
    }
    best
}


fn ordering_score(order: &Vec<String>,
    distances: &HashMap<String,HashMap<String,i64>>,
    valves: &HashMap<String,Valve>,
    mut time: i64) -> Option<i64> {
    let mut score = 0;
    let mut now = "AA";
    for next_location in order {
        time -= distances.get(now)
            .unwrap()
            .get(next_location).unwrap() +1;
        if time < 0 {
            return None
        }
        score += time * valves.get(next_location).unwrap().flow_rate;
        now = next_location;
    }
    Some(score)
}

fn parse_data(data: &str) -> HashMap<String,Valve> {
    let mut valves = HashMap::new();
    for line in data.lines() {
        let valve = line.parse::<Valve>().unwrap();
        valves.insert(valve.name.clone(), valve);
    }
    valves
}

fn distances(valves: &HashMap<String,Valve>) -> HashMap<String,HashMap<String,i64>> {
    let connections = valves.iter().map(|(a,b)| (a.to_owned(), b.tunnels.clone())).collect::<HashMap<_,_>>();
    let mut dist_by_origin = HashMap::new();
    let mut unconsidered = VecDeque::new();
    for (origin, neighbours) in connections.iter() {
        let mut distances = HashMap::new();
         for dest in neighbours{
            unconsidered.push_back((dest,1));
        }

        while let Some((next_dest, dist)) = unconsidered.pop_front() {
            if !distances.contains_key(next_dest) && next_dest != origin {
                distances.insert(next_dest.to_owned(), dist );
                for chained in connections.get(next_dest).unwrap() {
                    unconsidered.push_back((chained, dist+1));
                }
            }
        }
        dist_by_origin.insert(origin.to_owned(), distances);
    }

    dist_by_origin
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_part1() {
        let valves = parse_data(DATA);
        let distances = distances(&valves);
        let working_valves = valves.values()
            .filter(|v| v.flow_rate > 0)
            .map(|v| v.name.clone())
            .collect::<Vec<_>>();
        assert_eq!(part1(&valves, &distances, &working_valves), 1651);
    }

    #[test]
    fn test_part2() {
        let valves = parse_data(DATA);
        let distances = distances(&valves);
        let working_valves = valves.values()
            .filter(|v| v.flow_rate > 0)
            .map(|v| v.name.clone())
            .collect::<Vec<_>>();
        assert_eq!(part2(&valves, &distances, &working_valves), 1707);
    }
}
