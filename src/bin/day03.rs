use std::io::Read;
use aoc2025::run;
use std::io::BufRead;

fn main() {
    run(part01, part02)
}

fn part01<R: Read>(mut input: R) -> Result<i64, std::io::Error> {
    let mut total_joltage = 0;

    for line in std::io::BufReader::new(&mut input).lines() {
        let batteries: Vec<u8> = line?
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            .collect();

        let mut joltage = 0;
        for first in 0..batteries.len() - 1 {
            for second in first + 1..batteries.len() {
                let combined = batteries[first] * 10 + batteries[second];
                if combined > joltage {
                    joltage = combined;
                }
            }
        }

        total_joltage += joltage as i64;
    }

    Ok(total_joltage)
}

fn part02<R: Read>(mut _input: R) -> Result<i64, std::io::Error> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "Not implemented"))
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
        assert_eq!(part02(input()).unwrap(), 0);
    }
}
