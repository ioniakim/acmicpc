use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::BufWriter::new(io::stdout());

    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input)?;
        let sum: u8 = input.split_whitespace()
            .map(str::parse::<u8>)
            .sum::<Result<u8, _>>()?;
        input.clear();

        if sum == 0 {
            break;
        }
        writeln!(out, "{}", sum)?;
    }

    Ok(())
}
