use std::io;
use std::cmp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let values = io::read_to_string(io::stdin()).unwrap_or("".to_string()).split_whitespace()
        .take(4)
        .map(str::parse::<u16>)
        .collect::<Result<Vec<_>, _>>()?;

    let x = values[0];
    let y = values[1];
    let w = values[2];
    let h = values[3];

    let mut min = x;
    min = cmp::min(min, w - x);
    min = cmp::min(min, y);
    min = cmp::min(min, h - y);
    println!("{min}");
    Ok(())
}
