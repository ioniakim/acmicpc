use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let nums: Vec<i32> = input.split_whitespace()
        .map(|s| {
            s.chars().rev().collect::<String>()
        })
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    println!("{}", std::cmp::max(nums[0], nums[1]));

    Ok(())
}
