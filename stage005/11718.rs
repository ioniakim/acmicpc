use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::BufWriter::new(io::stdout());

    for line in io::stdin().lines() {
        let line = line?;
        writeln!(out, "{line}")?;
    }
    Ok(())
}
