use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::BufWriter::new(io::stdout());

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let n: usize = input.trim().parse()?;

    input.clear();
    io::stdin().read_line(&mut input)?;
    let numbers: Vec<i32> = input.split_whitespace()
        .take(n)
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let min = numbers.iter().min().unwrap();
    let max = numbers.iter().max().unwrap();

    writeln!(out, "{} {}", min, max)?;

    Ok(())
}
