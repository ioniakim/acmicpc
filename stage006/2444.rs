use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::BufWriter::new(io::stdout());

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse()?;

    let mut width = 1usize;
    (1..n).for_each(|_| {
        width += 2;
    });

    let mut prev: usize = 1;
    for _ in 0..n {
        writeln!(out, "{: ^width$}", "*".repeat(prev))?;
        prev += 2;
    }

    prev -= 2;
    for _ in 1..n {
        prev -= 2;
        writeln!(out, "{: ^width$}", "*".repeat(prev))?;
    }

    Ok(())
}
