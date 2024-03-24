use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const CHESS_PINS: [i8; 6] = [1, 1, 2, 2, 2, 8];

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let found: Vec<i8> = input.split_whitespace()
        .take(6)
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let answer = found.iter().zip(CHESS_PINS.iter())
        .map(|(&f, &c)| {
            (c - f).to_string()
        })
        .collect::<Vec<String>>()
        .join(" ");

    println!("{answer}");
    Ok(())
}
