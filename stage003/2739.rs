use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    let n: u8 = input.trim().parse()?;

    if n > 9 {
        return Err("Invalid input number".into());
    }

    for i in 1..10 {
        println!("{n} * {i} = {}", n * i);
    }

    Ok(())
}
