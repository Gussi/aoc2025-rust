use std::io::Read;
use aoc2025::run;

fn main() {
    run(part01, part02)
}

fn part01<R: Read>(mut input: R) -> Result<i64, std::io::Error> {
    let mut total_invalid_ids = 0;

    // get input, and split by commas
    let mut input_string = String::new();
    input.read_to_string(&mut input_string)?;
    let ranges: Vec<&str> = input_string.trim().split(',').collect();

    for range in ranges {
        let parts: Vec<&str> = range.split('-').collect();
        // We don't want to parse id to i32, as we need to do some string checks on it
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

fn part02<R: Read>(mut _input: R) -> Result<i64, std::io::Error> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "Not implemented"))
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
        // assert_eq!(part02(input()).unwrap(), 5);
    }
}
