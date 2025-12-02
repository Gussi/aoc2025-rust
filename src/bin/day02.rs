use std::io::Read;
use aoc2025::run;

struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(start: i64, end: i64) -> Self {
        Range { start, end }
    }
}

fn main() {
    run(part01, part02)
}

fn part01<R: Read>(mut input: R) -> Result<i64, std::io::Error> {
    let mut total_invalid_ids = 0;

    let mut input_string = String::new();
    input.read_to_string(&mut input_string)?;
    let ranges: Vec<&str> = input_string.trim().split(',').collect();

    for range in ranges {
        let parts: Vec<&str> = range.split('-').collect();

        let start_id = parts[0];
        let end_id = parts[1];

        for id in start_id.parse::<i64>().unwrap()..=end_id.parse::<i64>().unwrap() {
            let id_str = id.to_string();
            if is_valid_id(&id_str) {
                continue;
            }

            total_invalid_ids += id;
        }
    }

    Ok(total_invalid_ids)
}

fn part02<R: Read>(mut _input: R) -> Result<i64, std::io::Error> {
    let mut total_invalid_ids = 0;

    let mut input_string = String::new();
    _input.read_to_string(&mut input_string)?;

    for range in get_ranges(&input_string) {

        for id in range.start..=range.end {

            if is_valid_id_part02(id.to_string().as_str()) {
                continue;
            }

            total_invalid_ids += id;
        }
    }

    Ok(total_invalid_ids)
}

fn get_ranges(input: &str) -> Vec<Range> {
    input
        .trim()
        .split(',')
        .map(|range| {
            let parts: Vec<&str> = range.split('-').collect();
            Range::new(
                parts[0].parse::<i64>().unwrap(),
                parts[1].parse::<i64>().unwrap(),
            )
        })
        .collect()
}

fn is_valid_id(id: &str) -> bool {
    let len = id.len();
    if len % 2 != 0 {
        return true;
    }
    let half = len / 2;
    let first_half = &id[0..half];
    let second_half = &id[half..len];
    first_half != second_half
}

fn is_valid_id_part02(id: &str) -> bool {
    let len = id.len();

    for sub_len in 1..=len / 2 {
        if len % sub_len != 0 {
            continue;
        }
        let sub_str = &id[0..sub_len];
        let mut repeated = String::new();
        for _ in 0..(len / sub_len) {
            repeated.push_str(sub_str);
        }
        if repeated == id {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static [u8] {
        b"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
    }

    #[test]
    fn test_part01() {
        assert_eq!(part01(input()).unwrap(), 1227775554);
    }

    #[test]
    fn test_part02() {
        assert_eq!(part02(input()).unwrap(), 4174379265);
    }
}
