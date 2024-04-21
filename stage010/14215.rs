use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut laterals: Vec<u16> = input.split_whitespace()
        .flat_map(str::parse::<u16>)
        .collect::<Vec<_>>();

    laterals.sort_by(|a, b | b.cmp(a));

    let sum = laterals[1..].iter().sum::<u16>();
    let perimeter = if laterals[0] >= sum {
        sum + sum - 1
    } else {
        sum + laterals[0]
    };
    println!("{perimeter}");

    Ok(())
}
