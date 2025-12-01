use std::io::{BufRead};
use std::io::Read;

fn main() {
    let part = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: day01 <part>");
        std::process::exit(1);
    });

    let result = match part.as_str() {
        "1" => part01(std::io::stdin()),
        "2" => part02(std::io::stdin()),
        _ => {
            eprintln!("Invalid part specified. Please use '1' or '2'.");
            std::process::exit(1);
        }
    };

    match result {
        Ok(output) => println!("Result: {}", output),
        Err(e) => {
            eprintln!("Error occurred: {}", e);
            std::process::exit(1);
        }
    }
}

fn part01<R: Read>(mut input: R) -> Result<i32, std::io::Error> {
    let mut dial = 50;
    let mut counter = 0;

    for line in std::io::BufReader::new(&mut input).lines() {
        let prefix = &line.as_ref().unwrap()[0..1];
        let number: i32 = line.as_ref().unwrap()[1..].parse().unwrap();

        match prefix {
            "L" => dial -= number,
            "R" => dial += number,
            _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Unexpected prefix")),
        }

        if dial % 100 == 0 {
            counter += 1;
        }
    }

    return Ok(counter);
}

fn part02<R: Read>(mut input: R) -> Result<i32, std::io::Error> {
    let mut dial = 50;
    let mut counter = 0;

    for line in std::io::BufReader::new(&mut input).lines() {
        let prefix = &line.as_ref().unwrap()[0..1];
        let number: i32 = line.as_ref().unwrap()[1..].parse().unwrap();

        match prefix {
            "L" => {
                let start_r = if dial == 0 { 0 } else { 100 - dial };
                counter += (start_r + number) / 100;
                dial = (dial - number).rem_euclid(100);
            }
            "R" => {
                counter += (dial + number) / 100;
                dial = (dial + number) % 100;
            }
            _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Unexpected prefix")),
        }
    }

    return Ok(counter);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static [u8] {
        b"L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82"
    }

    #[test]
    fn test_part01() {
        assert_eq!(part01(input()).unwrap(), 3);
    }

    #[test]
    fn test_part02() {
        assert_eq!(part02(input()).unwrap(), 6);
    }
}
