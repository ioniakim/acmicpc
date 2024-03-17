use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse()?;

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;

        let num: Vec<u16> = input.split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        write!(out, "{}\n", num[0] + num[1])?;
    }

    Ok(())
}
