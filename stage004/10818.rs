use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::BufWriter::new(io::stdout());

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let n: usize = input.trim().parse()?;

    input.clear();
    io::stdin().read_line(&mut input)?;
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    input.split_whitespace()
        .take(n)
        .map(str::parse)
        .for_each(|v| {
            min = std::cmp::min(min, v.clone().unwrap());
            max = std::cmp::max(max, v.clone().unwrap());
        });

    writeln!(out, "{} {}", min, max)?;

    Ok(())
}
