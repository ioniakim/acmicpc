use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::BufWriter::new(io::stdout());

    let mut max = u32::MIN;
    let mut index = 0usize;
    for (i, line) in io::stdin().lines().enumerate() {
        let line = line?;
        let n: u32 = line.trim().parse()?;
        if max < n {
            max = n;
            index = i + 1;
        }
    }

    writeln!(out, "{max}")?;
    writeln!(out, "{index}")?;

    Ok(())
}
