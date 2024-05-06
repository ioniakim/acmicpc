use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>()?;

    let mut members = vec![];
    for line in std::io::stdin().lines().take(n) {
        let line = line?;
        let mut iter = line.split_whitespace();
        members.push((iter.next().unwrap().parse::<u8>()?,
            iter.next().unwrap().to_owned()));
    }

    members.sort_by_key(|e| e.0);

    let mut out = std::io::BufWriter::new(std::io::stdout());
    for m in members{
        writeln!(out, "{} {}", m.0, m.1)?;
    }

    Ok(())
}
