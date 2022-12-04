use std::io::Error;
use parse_display::{Display, FromStr};


#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{start}-{end}")]
struct SectionAssignment {
  start: i32,
  end: i32,
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{left},{right}")]
struct ElfPair {
  left: SectionAssignment,
  right: SectionAssignment,
}

impl SectionAssignment {
    fn contains (&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlap(&self, other: &Self) -> Option<Self> {
        if self.end < other.start || other.end < self.start {
            None
        } else {
            Some(SectionAssignment { start: self.start.max(other.start), end: self.end.min(other.end) })
        }
    }
}

fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("./day04/input.txt")?;
    let (part1, part2) = solve(&data);
    println!("Part 1: {part1}\nPart 2: {part2}");
    Ok(())
}

fn solve(data:&str) -> (i32, i32) {
    let (mut part1, mut part2) = (0,0);
    for elf_pair in data.split('\n').map(|l| l.parse::<ElfPair>().unwrap()) {
        part1 += (elf_pair.left.contains(&elf_pair.right) || elf_pair.right.contains(&elf_pair.left)) as i32;
        part2 += elf_pair.left.overlap(&elf_pair.right).is_some() as i32;
    }
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_p1() {
        assert_eq!(solve(DATA), (2,4));
    }
}