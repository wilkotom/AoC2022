use std::{io::Error, str::FromStr, collections::{HashMap, VecDeque}};
use itertools::Itertools;

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
        .map(|v| &v.name)
        .collect::<Vec<_>>();
    println!("Part 1: {:?}", part1(&valves, &distances, &working_valves));
    println!("Part 2: {:?}", part2(&valves, &distances, &working_valves));

    Ok(())
}

fn part1<'a>(valves: &HashMap<String,Valve>,distances: &HashMap<String,HashMap<String,i64>>, working_valves: &[&'a String]) -> i64 {
    let start = "AA".to_string();
    let mut cache: HashMap<(String, Vec<&String>, i64), Option<i64>> = HashMap::new();
    best_path_for_valves(&start, working_valves, distances, valves, 30, &mut cache).unwrap()
}

fn part2<'a>(valves: &HashMap<String,Valve>,distances: &HashMap<String,HashMap<String,i64>>, working_valves: &[&'a String]) -> i64 {

    let unchecked = working_valves.iter().map(|v| v.to_string()).collect::<Vec<_>>();
    let start = "AA".to_string();
    let mut best = 0;
    let mut cache: HashMap<(String, Vec<&String>, i64), Option<i64>> = HashMap::new();
    let mut path_scores = Vec::new();
    
    for combination in unchecked.iter().combinations(working_valves.len() /2) {
        if let Some(score) = best_path_for_valves(&start, &combination, distances, valves, 26, &mut cache) {
            path_scores.push((combination, score));
        }
    }

    for (i, (left_path, left_path_score)) in path_scores.iter().enumerate() {
        for (right_path, right_path_score) in path_scores[i+1..].iter(){
            if right_path.iter().all(|v| !left_path.contains(v)) {
                let score = left_path_score + right_path_score;
                best = best.max(score);
            }
        }
    }

    best
}


fn best_path_for_valves<'a>(start: &String, 
    to_visit: &[&'a String],
    distances: &HashMap<String,HashMap<String,i64>>,
    valves: &HashMap<String,Valve>,
    time: i64,
    cache: &mut HashMap<(String, Vec<&'a String>, i64), Option<i64>> ) -> Option<i64> {
    let cache_key = (start.to_owned(), to_visit.to_vec(), time );
    if let Some(entry) = cache.get(&cache_key) {
        return *entry;
    }
    let mut best_flowed = None;
    for next_room in to_visit.iter() {
        let time_taken =  distances.get(start).unwrap().get(*next_room).unwrap() +1;
        if time_taken > time {
            continue;
        }
        let mut flowed = (time - time_taken) * valves.get(*next_room).unwrap().flow_rate;
        let unvisited = to_visit.iter().filter(|x| *x != next_room).map(|x| x.to_owned()).collect::<Vec<_>>();
        if let Some(next_flowed) = best_path_for_valves(next_room, &unvisited, distances, valves, time - time_taken, cache) {
            flowed += next_flowed
        }
        best_flowed = best_flowed.max(Some(flowed));
    }
    cache.insert(cache_key, best_flowed );
    best_flowed
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
            .map(|v| &v.name)
            .collect::<Vec<_>>();
        assert_eq!(part1(&valves, &distances, &working_valves), 1651);
    }

    #[test]
    fn test_part2() {
        let valves = parse_data(DATA);
        let distances = distances(&valves);
        let working_valves = valves.values()
            .filter(|v| v.flow_rate > 0)
            .map(|v| &v.name)
            .collect::<Vec<_>>();
        assert_eq!(part2(&valves, &distances, &working_valves), 1707);
    }
}