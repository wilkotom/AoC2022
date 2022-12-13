use std::{io::Error, cmp::Ordering, str::FromStr, num::ParseIntError};

#[derive(Debug,Clone, PartialEq, Eq)]
enum Packet {
    Value(i32),
    Packet(Vec<Packet>)
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Value(s), Packet::Value(o)) => s.cmp(o),
            (Packet::Value(n), Packet::Packet(_)) => Packet::Packet(vec![Packet::Value(*n)]).cmp(other),
            (Packet::Packet(_), Packet::Value(n)) => self.cmp(&Packet::Packet(vec![Packet::Value(*n)])),
            (Packet::Packet(left), Packet::Packet(right)) => {
                for i in 0.. left.len().min(right.len()) {
                    match left[i].cmp(&right[i]) {
                        Ordering::Less => {
                            return Ordering::Less
                        },
                        Ordering::Greater => {
                            return Ordering::Greater
                        }
                        _ => {}
                    }
                }
                left.len().cmp(&right.len()) 
            }
        }
    }
}

impl FromStr for Packet {
    fn from_str(line: &str) ->  Result<Self, Self::Err> {

        fn process_tokens(tokens: &mut Vec<&str>) -> Result<Vec<Packet>, ParseIntError> {
            let mut result = Vec::new();
            while !tokens.is_empty() {
                match tokens.pop() {
                    Some("]") => {
                        return Ok(result);
                    },
                    Some("[") => {
                        result.push(Packet::Packet(process_tokens(tokens)?));
                    }
                    Some("") => {}
                    Some(n) => {
                        result.push(Packet::Value(n.parse::<i32>()?));
                    }
                    None => unreachable!()
                };
            }
            Ok(result)
        }

        let binding = line.replace('[', "[,").replace(']', ",]");
        let mut tokens = binding.split(',').rev().collect::<Vec<_>>();
        Ok(Packet::Packet(process_tokens(&mut tokens)?))
    }
        
    type Err = ParseIntError;

}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("./day13/input.txt")?;
    let part1 = part1(&data)?;
    let part2 = part2(&data)?;
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);

    Ok(())
}

fn part1(data: &str) -> Result<usize,Error> {
    let mut total  = 0;
    for (i,pair) in data.split("\n\n").enumerate() {
        let mut lines =  pair.lines();
        let left = lines.next().unwrap().parse::<Packet>().unwrap();
        let right = lines.next().unwrap().parse::<Packet>().unwrap();
        if left < right {
            total += i+1
        }
    }
    Ok(total)
}


fn part2(data: &str) -> Result<usize,Error> {

    let mut all_packets = data.lines().filter(|l| !l.is_empty()).map(|l| l.parse::<Packet>().unwrap()).collect::<Vec<_>>();
    let sep_2 = "[[2]]".parse::<Packet>().unwrap();
    let sep_6 = "[[6]]".parse::<Packet>().unwrap();
    all_packets.push(sep_2.clone());
    all_packets.push(sep_6.clone());

    all_packets.sort();

    let pos_2 = all_packets.iter().position(|p| p == &sep_2).unwrap() +1; 
    let pos_6 = all_packets.iter().position(|p| p == &sep_6).unwrap() +1; 
    Ok(pos_2 * pos_6)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part1() {
    
        assert_eq!(part1(DATA).unwrap(), 13);
    }

    #[test]
    fn test_part2() {
    
        assert_eq!(part2(DATA).unwrap(), 140);
    }
}