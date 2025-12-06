use std::io::Read;
use aoc2025::run;
use std::io::BufRead;
use num_bigint::BigInt;

fn main() {
    run(part01, part02)
}

fn part01<R: Read>(mut input: R) -> Result<i64, std::io::Error> {
    let mut total_joltage = 0;

    for line in std::io::BufReader::new(&mut input).lines() {
        let mut digits = BigInt::parse_bytes(line?.as_bytes(), 10).unwrap();

        while digits.to_string().len() > 2 {
            let biggest = find_biggest_number_by_removing_one_digit(&digits);
            digits = biggest;
        }

        total_joltage += digits.to_u64_digits().1[0] as i64;
    }

    Ok(total_joltage)
}

fn find_biggest_number_by_removing_one_digit(digits: &BigInt) -> BigInt {
    let digits_str = digits.to_string();
    let mut max_number = BigInt::from(0);

    for i in 0..digits_str.len() {
        let mut new_number_str = String::new();
        for (j, c) in digits_str.chars().enumerate() {
            if j != i {
                new_number_str.push(c);
            }
        }
        let new_number = BigInt::parse_bytes(new_number_str.as_bytes(), 10).unwrap();
        if new_number > max_number {
            max_number = new_number;
        }
    }

    max_number
}

fn part02<R: Read>(mut input: R) -> Result<i64, std::io::Error> {
    let mut total_joltage = 0;

    for line in std::io::BufReader::new(&mut input).lines() {
        let mut digits = BigInt::parse_bytes(line?.as_bytes(), 10).unwrap();

        while digits.to_string().len() > 12 {
            let biggest = find_biggest_number_by_removing_one_digit(&digits);
            digits = biggest;
        }

        total_joltage += digits.to_u64_digits().1[0] as i64;
    }

    Ok(total_joltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static [u8] {
        b"987654321111111\n811111111111119\n234234234234278\n818181911112111"
    }

    #[test]
    fn test_part01() {
        assert_eq!(part01(input()).unwrap(), 357);
    }

    #[test]
    fn test_part02() {
        assert_eq!(part02(input()).unwrap(), 3121910778619);
    }
}
