use anyhow::Error;
// use parse_display::{Display, FromStr};

#[derive(PartialEq, Debug)]
enum Instruction {
    NoOp,
    AddX(i32)
}


fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("./day10/input.txt")?;
    let (part1, part2) = run_program(&parse_instructions(&data));
    println!("Part 1: {}", part1);
    println!("Part 2: \n{}\n", part2);
    Ok(())
}

fn run_program(program: &[Instruction]) -> (i32, String) {
    let mut x_register: i32 = 1;
    let mut part1_answer = 0;
    let mut display = String::new();

    let interesting_cycles = [20, 60, 100, 140, 180, 220];
    
    for (cycle_timer, line) in program.iter().enumerate() {
        if cycle_timer % 40 == 0 {
            display.push('\n')
        }
        if interesting_cycles.contains(&(cycle_timer+1)) {
            part1_answer += (cycle_timer as i32 +1) * x_register;
        }
        display.push(
            if (x_register - ((cycle_timer) % 40) as i32 ).abs() <=1 
                    {'█'}  else  {' '});
        match line {
            Instruction::NoOp => {}
            Instruction::AddX(n) => {
                x_register += *n; 
            }
        }
    }
    (part1_answer, display)

}

fn parse_instructions(data:&str) -> Vec<Instruction> {
    let mut instructions = vec![];
    for line in data.lines() {
        match line.strip_prefix("addx ") {
            None =>{ instructions.push(Instruction::NoOp)}
            Some(l) => {
                instructions.push(Instruction::NoOp);
                instructions.push(Instruction::AddX(l.parse::<i32>().unwrap()));
            }
        }
    }
    instructions
}


#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_parts_1_and_2 (){
        let prog = parse_instructions(DATA);
        let (part1, part2) = run_program(&prog);
        let part2_expected = "
██  ██  ██  ██  ██  ██  ██  ██  ██  ██  
███   ███   ███   ███   ███   ███   ███ 
████    ████    ████    ████    ████    
█████     █████     █████     █████     
██████      ██████      ██████      ████
███████       ███████       ███████     ";
        assert_eq!(part1, 13140);
        assert_eq!(part2, part2_expected);


    }

}