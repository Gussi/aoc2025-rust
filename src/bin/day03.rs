use std::io::Read;
use aoc2025::run;
use std::io::BufRead;

struct BatteryBank {
    batteries: Vec<u8>,
}

impl BatteryBank {
    fn new(batteries: Vec<u8>) -> Self {
        Self { batteries }
    }

    fn max_joltage(&self, digits: usize) -> u64 {
        fn helper(batteries: &[u8], digits: usize, start: usize, current: u64, max: &mut u64) {
            if digits == 0 {
                if current > *max {
                    *max = current;
                }
                return;
            }

            for i in start..batteries.len() {
                let next = current * 10 + batteries[i] as u64;
                helper(batteries, digits - 1, i + 1, next, max);
            }
        }

        let mut max_joltage = 0;
        helper(&self.batteries, digits, 0, 0, &mut max_joltage);
        max_joltage
    }
}


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

        let bank = BatteryBank::new(batteries);

        total_joltage += bank.max_joltage(2) as i64;
    }

    Ok(total_joltage)
}

fn part02<R: Read>(mut input: R) -> Result<i64, std::io::Error> {
    let mut total_joltage = 0;

    for line in std::io::BufReader::new(&mut input).lines() {
        let batteries: Vec<u8> = line?
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            .collect();

        let bank = BatteryBank::new(batteries);

        total_joltage += bank.max_joltage(12) as i64;
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
