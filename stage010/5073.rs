use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {

    for line in io::stdin().lines() {
        let line = line?;
        let laterals = line.split_whitespace()
            .map(str::parse::<u16>)
            .collect::<Result<Vec<_>, _>>()?;

        let triangle = match laterals[..] {
            [0, 0, 0] => break,
            [a, b, c] if !is_valid(a, b, c) => "Invalid",
            [a, b, c] if a == b && b == c && c == a => "Equilateral",
            [a, b, c] if a == b || b == c || c == a => "Isosceles",
            [a, b, c] if a != b && b != c && c != a => "Scalene",
            _ => "Invalid",
        };

        println!("{triangle}");
    }
    Ok(())
}

fn is_valid(a: u16, b: u16, c: u16) -> bool {
    let mut laterals = [a, b, c];
    laterals.sort();
    laterals[2] < laterals[0] + laterals[1]
}
