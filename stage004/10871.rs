use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::BufWriter::new(io::stdout());

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let info: Vec<i32> = input.split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    if info.len() != 2 {
        return Err("Invalid format".into());
    }

    let x = info[1];

    input.clear();
    io::stdin().read_line(&mut input)?;
    let result = input.split_whitespace()
        .flat_map(str::parse::<i32>)
        .filter(|&r| r < x)
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    writeln!(out, "{result}")?;

    Ok(())
}
