use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut iter = input.split_whitespace();
    let climb = iter.next().unwrap_or("No Climb Argument").parse::<u32>()?;
    let slip = iter.next().unwrap_or("No Slip Argument").parse::<u32>()?;
    let tree = iter.next().unwrap_or("No Tree Argument").parse::<u32>()?;

    let mut height = climb;
    let mut days = 1;
    while height < tree {
        height -= slip;
        height += climb;
        days += 1;
    }

    println!("{days}");

    Ok(())
}
