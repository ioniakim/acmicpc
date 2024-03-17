use std::fmt::Write;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let n: usize = input.trim().parse()?;

    let bytes = n / 4;

    let mut output = String::with_capacity("long ".len() * (bytes - 1) + "long int".len());
    (0..(bytes - 1)).for_each(|_| write!(output, "long ").expect("Failed"));
    write!(output, "long int")?;
    println!("{output}");

    Ok(())
}
