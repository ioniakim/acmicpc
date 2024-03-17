use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::BufWriter::new(io::stdout().lock());

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let count: usize = input.trim().parse()?;

    (1..=count).for_each(|i| {
        writeln!(out, "{:>count$}", "*".repeat(i)).expect("Failed");
    });

    Ok(())
}
