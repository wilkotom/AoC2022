use std::error::Error;
use parse_display::{Display, FromStr};
use anyhow::{anyhow, Result};


#[derive(Debug)]
struct Directory {
    _name: String,
    files: Vec<FileMetadata>,
    children: Vec<Directory>
}

impl Directory {
    fn total_size(&self) -> i64 {
        let files = self.files.iter().map(|f| f.size).sum::<i64>();
        let dirs = self.children.iter().map(|d| d.total_size()).sum::<i64>();
        files + dirs
    }

    fn part1(&self) -> i64 {
        let size = self.total_size();
        self.children.iter().map(|d| d.part1()).sum::<i64>() + if size < 100000 {size} else {0}
    }

    fn part2(&self, desired: i64) -> Option<i64> {
        let size = self.total_size();
        if size < desired {
            None
        } else if let Some(smaller) = self.children.iter().filter_map(|d| d.part2(desired)).min() {
            Some(smaller)
        } else {
            Some(size)
        }
    }
}

#[derive(Display, FromStr, Debug)]
#[display("{size} {_name}")]
struct FileMetadata {
    _name: String,
    size: i64
}


fn main() -> Result<(), Box<dyn Error>> {
    let data = std::fs::read_to_string("./day07/input.txt")?;
    if let Ok((part1, part2)) = solve(&data) {
        println!("Part 1: {}\nPart 2: {}", part1, part2);
    }
    Ok(())
}

fn solve(data: &str) -> Result<(i64, i64)> {
    let mut instruction_stack = data.lines().rev().collect::<Vec<_>>();
    let tree = parse_tree(&mut instruction_stack)?;
    let needed = 30000000 - (70000000 - tree.total_size());
    Ok((tree.part1(), if let Some(n) = tree.part2(needed) {n} else {0}))
}

fn parse_tree(instructions: &mut Vec<&str>) -> Result<Directory> {
    let instruction = instructions.pop().unwrap();
    // Starts with "$ cd <blah>" to enter a new directory
    // "$ cd .." indicates we've finished
    if let Some(dirname) = instruction.strip_prefix("$ cd ") {
        let mut next_instr = instructions.pop().unwrap_or("$ cd ..");
        let mut files = vec![];
        let mut children = vec![];
        while next_instr != "$ cd .." {
            if next_instr.starts_with("$ cd ") {
                // going down a level, return the instruction to the stack for the 
                // next level of recursion to consume
                instructions.push(next_instr);
                children.push(parse_tree(instructions)?);
            } else if let Ok(file) = next_instr.parse::<FileMetadata>(){
                files.push(file);
            }
            next_instr = instructions.pop().unwrap_or("$ cd ..");
        }
        Ok(Directory { _name: dirname.to_string(), files, children })
    } else {
        Err(anyhow!("Can't identify CWD from: {:?}", instruction))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn test_parts (){
        let mut instruction_stack = DATA.lines().rev().collect::<Vec<_>>();
        let tree = parse_tree(&mut instruction_stack).unwrap();
        let tree_size = tree.total_size();
        assert_eq!(tree_size, 48381165);
        assert_eq!(tree.part1(), 95437);
        let needed = 30000000 - (70000000 - tree_size);
        assert_eq!(tree.part2(needed), Some(24933642));
     }

}