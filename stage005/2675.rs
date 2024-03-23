use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::BufWriter::new(io::stdout());

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse()?;

    for line in io::stdin().lines().take(n) {
        let line = line?;
        let input: Vec<_> = line.split_whitespace()
            .collect();

        let repeat: u8 = input[0].parse()?;

        for c in input[1].chars() {
            for _ in 0..repeat {
                write!(out, "{c}")?;
            }
        }
        writeln!(out)?;
    }

    Ok(())
}
