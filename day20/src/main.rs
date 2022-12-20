use std::io::Error;

fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("./day20/input.txt")?;
    let numbers = data.lines().map(|l| l.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let part1_mixed = mix(&numbers, 1, 1);
    let zero_index = part1_mixed.iter().position(|n|n == &0).unwrap();
    println!("Part 1: {}",  part1_mixed[(zero_index +1000) % part1_mixed.len()]+ part1_mixed[(zero_index + 2000) % part1_mixed.len()]+part1_mixed[(zero_index + 3000) % part1_mixed.len()]);
    let part2_mixed = mix(&numbers, 10, 811589153);
    let zero_index = part2_mixed.iter().position(|n|n == &0).unwrap();
    println!("Part 2: {}",  part2_mixed[(zero_index +1000) % part2_mixed.len()]+ part2_mixed[(zero_index + 2000) % part2_mixed.len()]+part2_mixed[(zero_index + 3000) % part2_mixed.len()]);
    Ok(())
}

fn mix(numbers: &[i64], repetitions: usize, decryption_key: i64) -> Vec<i64> {
    let mut results = numbers.iter().map(|n| (n*decryption_key)).enumerate().collect::<Vec<_>>();
    for _ in 0..repetitions {
        for number_pos_pair in numbers.iter().map(|n| n* decryption_key ).enumerate() {
            let current_index = results.iter().position(|x| x == &number_pos_pair).unwrap();
            results.remove(current_index);
            let new_index = ((current_index as i64 + number_pos_pair.1).rem_euclid(results.len() as i64)) as usize;
            results.insert(new_index, number_pos_pair);
        }
    }
    results.iter().map(|r| r.1).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_day1() {
        let numbers = DATA.lines().map(|l| l.parse::<i64>().unwrap()).collect::<Vec<_>>();
        let mixed = mix(&numbers, 1, 1);
        assert_eq!(mixed, vec![-2, 1, 2, -3, 4, 0, 3]);
    }

    #[test]
    fn test_day2() {
        let numbers = DATA.lines().map(|l| l.parse::<i64>().unwrap()).collect::<Vec<_>>();
        let mixed = mix(&numbers, 10, 811589153);
        assert_eq!(mixed, vec![0, -2434767459, 1623178306, 3246356612, -1623178306, 2434767459, 811589153]);
    }
}