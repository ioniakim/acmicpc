use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let height = input.trim().parse::<u16>()?;

    input.clear();
    io::stdin().read_line(&mut input)?;
    let width = input.trim().parse::<u16>()?;

    let area = height * width;
    println!("{area}");
    Ok(())
}
