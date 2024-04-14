/// n*A - B*(n - 1) >= V
/// n*A - n*B + B >= V
/// n*(A - B) >= V - B
/// n >= (V - B) / (A - B)
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut iter = input.split_whitespace();
    let climb = iter.next().unwrap_or("").parse::<u32>()?;
    let slip = iter.next().unwrap_or("").parse::<u32>()?;
    let tree = iter.next().unwrap_or("").parse::<u32>()?;

    let days = (tree - slip).div_ceil(climb - slip);

    println!("{days}");

    Ok(())
}
