use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    let n: u32 = input.trim().parse()?;

    println!("{}", (1..=n).sum::<u32>());

    Ok(())
}
