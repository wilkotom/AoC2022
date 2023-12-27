use std::{collections::HashSet, io::Error};

fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("./day06/input.txt")?;
    println!("Part 1: {}", part1(&data, 4).unwrap());
    println!("Part 2: {}", part1(&data, 14).unwrap());
    Ok(())
}

fn part1(data:&str, length: usize) -> Option<usize> {
    (0..data.len())
        .map(|x| data[x..x+length]
        .chars()
        .collect::<HashSet<char>>().len())
        .position(|n| n == length).map(|n| n + length)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_p1_1() {
        //assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb",4), Some(7));
    }

    #[test]
    fn test_p1_2() {
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz",4 ), Some(5));
    }

    #[test]
    fn test_p1_3() {
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg",4), Some(6));
    }
    #[test]
    fn test_p1_4() {
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",4), Some(10));
    }
    #[test]
    fn wtest_p1_5() {
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",4), Some(11));
    }


    #[test]
    fn test_p2_1() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb",14), Some(19));
    }

    #[test]
    fn test_p2_2() {
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz",14 ), Some(23));
    }

    #[test]
    fn test_p2_3() {
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg",14), Some(23));
    }
    #[test]
    fn test_p2_4() {
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",14), Some(29));
    }
    #[test]
    fn test_p2_5() {
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",14), Some(26));
    }
}