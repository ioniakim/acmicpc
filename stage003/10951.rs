use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::BufWriter::new(io::stdout());

    for line in io::stdin().lines() {
        let line = line?;
        let sum = line.split_whitespace()
            .map(str::parse::<u8>)
            .sum::<Result<u8, _>>()?;
        writeln!(out, "{}", sum)?;
    }
    Ok(())
}
