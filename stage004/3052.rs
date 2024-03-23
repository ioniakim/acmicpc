use std::io;
use io::Write;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::BufWriter::new(io::stdout());

    let results = io::stdin().lines()
        .take(10)
        .flatten()  // Turn Result into its value String
        .map(|s| s.parse::<u16>().unwrap() % 42)
        .collect::<HashSet<_>>();

    writeln!(out, "{}", results.len())?;

    Ok(())
}
