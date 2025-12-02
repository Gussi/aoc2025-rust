pub fn run(
    part01: fn(std::io::Stdin) -> Result<i64, std::io::Error>,
    part02: fn(std::io::Stdin) -> Result<i64, std::io::Error>,
) {
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
            println!("Error occurred: {}", e);
            std::process::exit(1);
        }
    }
}
