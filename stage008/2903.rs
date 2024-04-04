use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let round = input.trim().parse()?;
    let base = 2u32.pow(round) + 1;
    println!("{}", base.pow(2));
    Ok(())
}
