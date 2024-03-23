use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse()?;

    input.clear();
    io::stdin().read_line(&mut input)?;
    let sum = input.trim().chars()
        .take(n)
        .map(|c| c.to_digit(10))
        .sum::<Option<u32>>().ok_or::<String>("Failed".into())?;
    println!("{sum}");
    Ok(())
}
