use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let input = io::read_to_string(io::stdin())?;
    let angles = input.split_whitespace()
        .map(str::parse::<u16>)   // The filter_map method requires the Option returned by the function parameter instead of Result
        .collect::<Result<Vec<_>, _>>()?;

    let triangle = match angles[..] {
        [60, 60, 60] => "Equilateral",
        [a, b, c] if a + b + c == 180 && (a == b) || (b == c) || (c == a) => "Isosceles",
        [a, b, c] if a + b + c == 180 && (a != b) && (b != c) && (c !=a ) => "Scalene",
        _ => "Error",
    };
    println!("{triangle}");
    Ok(())
}
