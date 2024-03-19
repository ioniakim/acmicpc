use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut out = io::BufWriter::new(io::stdout());
    let mut input = String::new();

    // read N
    io::stdin().read_line(&mut input)?;

    input.clear();
    io::stdin().read_line(&mut input)?;
    let numbers: Vec<i8> = input.split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    input.clear();
    io::stdin().read_line(&mut input)?;
    let v: i8 = input.trim().parse()?;

    let mut count = 0u8;

    for &i in &numbers {
        if v == i {
            count += 1;
        }
    }

    writeln!(out, "{}", count)?;
    Ok(())
}
