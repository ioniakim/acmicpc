use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::BufWriter::with_capacity(1024 * 1024, io::stdout());
    let mut count = [0usize; 10_001];

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>()?;
    for line in io::stdin().lines().take(n) {
        let line = line?;
        count[line.parse::<usize>()?] += 1;
    }

    for (v, &c) in count.iter().enumerate() {
        for _ in 0..c {
            writeln!(out, "{v}")?;
        }
    }
    Ok(())
}
