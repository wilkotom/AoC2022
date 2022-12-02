use std::{io::Error, cmp::Ordering};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors
}

impl Hand {
    fn score(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

enum GameResult {
    Win,
    Loss,
    Draw
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Hand::Rock => match other {
                Hand::Rock => Ordering::Equal,
                Hand::Paper => Ordering::Less,
                Hand::Scissors => Ordering::Greater,
            },
            Hand::Paper => match other {
                Hand::Rock => Ordering::Greater,
                Hand::Paper => Ordering::Equal,
                Hand::Scissors => Ordering::Less,
            },
            Hand::Scissors => match other {
                Hand::Rock => Ordering::Less,
                Hand::Paper => Ordering::Greater,
                Hand::Scissors => Ordering::Equal,
            },
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn main() ->  Result<(), Error>  {
    let data = std::fs::read_to_string("./day02/input.txt")?;
    let (part1, part2) = play_games(&data);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn play_games(data: &str) -> (i32,i32) {
    let mut p1_score = 0;
    let mut p2_score = 0;
    for line in data.split('\n') {
        let mut chars = line.chars();
        let opponent = parse_hand(&chars.next());
        let player_char = chars.last();
        let part1_player = parse_hand(&player_char);
        let part2_outcome = match player_char {
            Some('X') => GameResult::Loss,
            Some('Y') => GameResult::Draw,
            Some('Z') => GameResult::Win,
            _ => unimplemented!(),
        };
        let part2_player = match part2_outcome {
            GameResult::Win => match opponent {
                                Hand::Rock => Hand::Paper,
                                Hand::Paper => Hand::Scissors,
                                Hand::Scissors => Hand::Rock,
            },
            GameResult::Loss => match opponent {
                                Hand::Rock => Hand::Scissors,
                                Hand::Paper => Hand::Rock,
                                Hand::Scissors => Hand::Paper,
                                },
            GameResult::Draw =>  opponent,
        };
        p1_score += part1_player.score() + match part1_player.cmp(&opponent) {
                Ordering::Less => 0,
                Ordering::Equal => 3,
                Ordering::Greater => 6,
        };
        p2_score += part2_player.score() + match part2_player.cmp(&opponent) {
                Ordering::Less => 0,
                Ordering::Equal => 3,
                Ordering::Greater => 6,
        };
    }
    (p1_score, p2_score)
}

fn parse_hand(c: &Option<char>) -> Hand {
    match c {
        Some('A') | Some('X') => Hand::Rock,
        Some('B') | Some('Y') => Hand::Paper,
        Some('C') | Some('Z') => Hand::Scissors,
        _ => unimplemented!(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "A Y\nB X\nC Z";

    #[test]
    fn test_rock_paper_scissors() {
        let (part1_res, part2_res) =play_games(DATA);
        assert_eq!( part1_res, 15);
        assert_eq!( part2_res, 12);
    }
}