use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::BufWriter::new(io::stdout());
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse()?;

    for line in io::stdin().lines().take(n) {
        let line = line?;
        let s = line.trim().to_owned();
        let first = s.chars().nth(0).ok_or::<String>("first".into())?;
        let last = s.chars().nth_back(0).ok_or::<String>("last".into())?;

        writeln!(out, "{first}{last}")?;
    }

    Ok(())
}
