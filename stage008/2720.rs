use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const QUARTER: u16 = 25;
    const DIME: u16 = 10;
    const NICKEL: u16 = 5;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let count: usize = input.trim().parse()?;

    for line in io::stdin().lines().take(count) {
        let line = line?;

        let mut changes: u16 = line.parse()?;

        let quarters = changes / QUARTER;
        changes %= QUARTER;
        let dimes = changes / DIME;
        changes %= DIME;
        let nickels = changes / NICKEL;
        changes %= NICKEL;

        println!("{} {} {} {}", quarters, dimes, nickels, changes);
    }

    Ok(())
}
