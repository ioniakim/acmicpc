use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse()?;

    input.clear();
    io::stdin().read_line(&mut input)?;
    let sum: u16 = input.trim().chars()
        .take(n)
        .map(|c| c as u16 - '0' as u16)
        .sum::<u16>();
    println!("{sum}");
    Ok(())
}
