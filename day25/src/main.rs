use std::io::Error;

fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("./day25/input.txt")?;
    let total = data.lines().map(snafu_to_decimal).sum::<i64>();
    println!("{}", decimal_to_snafu(total));
    Ok(())
}
fn snafu_to_decimal(snafu: &str) -> i64 {
    let mut total = 0;
    for (i,c) in snafu.chars().rev().enumerate() {
        let column = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            _ => unimplemented!()
        };
        total += column * i64::pow(5, i as u32);
    }
    total
}

fn decimal_to_snafu(mut number: i64) -> String {
    let mut snafu_digits = Vec::new();
    while number > 0 {
        let remainder = number % 5;
        snafu_digits.push(match remainder {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => unreachable!()
        });
        if remainder >= 3 {
            number +=  5
        } 
        number /= 5;
    }
    snafu_digits.iter().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_snafu_parser() {
        assert_eq!(snafu_to_decimal("2=-01"), 976);
        assert_eq!(snafu_to_decimal("1=-0-2"), 1747);
        assert_eq!(snafu_to_decimal("12111"), 906);
        assert_eq!(snafu_to_decimal("2=0="), 198);
        assert_eq!(snafu_to_decimal("21"), 11);
        assert_eq!(snafu_to_decimal("2=01"), 201);
        assert_eq!(snafu_to_decimal("111"), 31);
        assert_eq!(snafu_to_decimal("20012"), 1257);
        assert_eq!(snafu_to_decimal("112"), 32);
        assert_eq!(snafu_to_decimal("1=-1="), 353);
        assert_eq!(snafu_to_decimal("1-12"), 107);
        assert_eq!(snafu_to_decimal("12"), 7);
        assert_eq!(snafu_to_decimal("1="), 3);
        assert_eq!(snafu_to_decimal("122"), 37);
    }

    #[test]
    fn test_snafu_generator() {
        assert_eq!(decimal_to_snafu(976), "2=-01".to_string());
        assert_eq!(decimal_to_snafu(625), "10000".to_string());
        assert_eq!(decimal_to_snafu(1747), "1=-0-2".to_string());
        assert_eq!(decimal_to_snafu(906), "12111".to_string());
        assert_eq!(decimal_to_snafu(198), "2=0=".to_string());
        assert_eq!(decimal_to_snafu(11), "21".to_string());
        assert_eq!(decimal_to_snafu(201), "2=01".to_string());
        assert_eq!(decimal_to_snafu(31), "111".to_string());
        assert_eq!(decimal_to_snafu(1257), "20012".to_string());
        assert_eq!(decimal_to_snafu(32), "112".to_string());
        assert_eq!(decimal_to_snafu(353), "1=-1=".to_string());
        assert_eq!(decimal_to_snafu(107), "1-12".to_string());
        assert_eq!(decimal_to_snafu(7), "12".to_string());
        assert_eq!(decimal_to_snafu(3), "1=".to_string());
        assert_eq!(decimal_to_snafu(37), "122".to_string());
    }
}