use std::io::BufRead;

fn main() {
    let mut dial = 50;
    let mut counter = 0;

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let prefix = &line.as_ref().unwrap()[0..1];
        let number: i32 = line.as_ref().unwrap()[1..].parse().unwrap();

        match prefix {
            "L" => dial -= number,
            "R" => dial += number,
            _ => panic!("Unexpected prefix"),
        }

        if dial % 100 == 0 {
            counter += 1;
        }
    }

    print !("{}", counter);
}
