use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::BufWriter::new(io::stdout());

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let height: usize = input.trim().parse()?;

    let width = (2 * height) - 1;

    for i in 0..height {
        let count = i * 2 + 1;
        writeln!(out, "{: ^width$}", "*".repeat(count))?;
    }

    for i in (0..height - 1).rev() {
        let count = i * 2 + 1;
        writeln!(out, "{: ^width$}", "*".repeat(count))?;
    }

    Ok(())
}
