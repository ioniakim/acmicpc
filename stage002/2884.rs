use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    let nums: Vec<i8> = input.split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let mut adjusted_hour = nums[0];
    let mut adjusted_minute = nums[1];

    if adjusted_minute < 45 {
        adjusted_hour = adjusted_hour - 1;
        adjusted_minute = 60 - 45 + adjusted_minute;
    } else {
        adjusted_minute -= 45;
    }

    if adjusted_hour < 0 {
        adjusted_hour = 23;
    }

    println!("{adjusted_hour} {adjusted_minute}");

    Ok(())
}
