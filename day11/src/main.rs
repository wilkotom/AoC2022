use std::{io::Error};

#[derive(Debug, Clone,Copy, PartialEq, Eq)]
enum Operation {
    Multiply,
    Add,
    Squared,
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct Monkey{
    items: Vec<i128>,
    operator: Operation,
    operatee: i128,
    test_divisor: i128,
    true_dest: usize,
    false_dest: usize
}  


fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("./day11/input.txt")?;
    let monkeys = data.split("\n\n").map(parse_monkey).collect::<Vec<_>>();
    let part1 = solution(monkeys.clone(), 20, true);
    let part2 = solution(monkeys, 10000, false);

    println!("Part 1: {:?}\nPart 2: {}", part1, part2);
    Ok(())
}

fn solution(mut monkeys: Vec<Monkey>, cycles: usize, part1:bool) -> i128 {
    let mut inspection_count = vec![0_i128; monkeys.len()];
    let lcm_divisors:i128 = monkeys.iter().map(|m| m.test_divisor).product();
    /*  Part 2: to stop worry_level escalating to the point it can't be stored in a number, 
        we need to truncate it. Because each monkey performs a predictable operation, 
        addition or multiplication, there will be many numbers for which all monkeys behave 
        the same way.

        Each of the monkeys applies a simple function, such as addition or multiplication,
        then divides by a particular prime; hence we can multiply each of these numbers 
        together to get the point at which the rule behaviour repeats.

        For each possible worry level, if f is the operation the monkey applies,

        f(worry_level) % monkey_divisor == f(worry_level % lcm_divisorts) % monkey_divisor

        is guaranteed to hold true. 
    
     */

    for _ in 0.. cycles{
        for i in 0.. monkeys.len() {
            let monkey = monkeys.get(i).unwrap().clone();
            for item in monkey.items {
                // let item = monkey.items.pop_front().unwrap();
                inspection_count[i] +=1;
                let worry_level = (match monkey.operator {
                    Operation::Multiply => item * monkey.operatee,
                    Operation::Add => item + monkey.operatee,
                    Operation::Squared => item * item,
                } / if part1 {3} else {1}) % lcm_divisors;
                if worry_level % monkey.test_divisor == 0 {
                    monkeys.get_mut(monkey.true_dest).unwrap().items.push(worry_level);
                } else {
                    monkeys.get_mut(monkey.false_dest).unwrap().items.push(worry_level);
                }

            }
            monkeys[i].items = Vec::new();
        }
    }
    inspection_count.sort_by(|a,b| b.cmp(&a));
    inspection_count[0] * inspection_count[1]
}


fn parse_monkey(monkey_str: &str) -> Monkey {
    let mut lines = monkey_str.lines();
    lines.next();
    let starting_items = lines.next().unwrap()[18..].split(", ").map(|x| x.parse::<i128>().unwrap()).collect::<Vec<_>>();
    let oper_line = lines.next().unwrap();
    let operator;
    let operatee;
    if &oper_line[23..] == "* old" {
        operator = Operation::Squared;
        operatee = 0;
    } else {
        operator = if oper_line.chars().nth(23).unwrap() == '+' {Operation::Add} else {Operation::Multiply};
        operatee = oper_line[25..].parse::<i128>().unwrap();
    }
    let test_divisor = lines.next().unwrap()[21..].parse::<i128>().unwrap();
    let true_dest = lines.next().unwrap()[29..].parse::<usize>().unwrap();
    let false_dest = lines.next().unwrap()[30..].parse::<usize>().unwrap();


    Monkey { items: starting_items, operator, operatee, test_divisor, true_dest, false_dest }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
  ";

    #[test]
    fn test_day1() {
        let monkeys = DATA.split("\n\n").map(parse_monkey).collect::<Vec<_>>();
        let part1 = solution(monkeys.clone(), 20, true);
        assert_eq!(part1,10605);
    }

    #[test]
    fn test_part2() {
        let monkeys = DATA.split("\n\n").map(parse_monkey).collect::<Vec<_>>();
        let part2 = solution(monkeys.clone(), 10000, false);
        assert_eq!(part2, 2713310158);
    }
}