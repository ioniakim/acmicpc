use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>()?;

    input.clear();
    std::io::stdin().read_line(&mut input)?;
    let origin = input.split_whitespace().take(n)
        .map(str::parse::<isize>)
        .collect::<Result<Vec<_>, _>>()?;

    let mut ordered = origin.clone();
    ordered.sort();

    ordered.dedup();

    let mut out = std::io::BufWriter::new(std::io::stdout());
    for e in origin {
        if let Ok(i) = ordered.binary_search(&e) {
            write!(out, "{} ", i)?
        }
    }
    writeln!(out)?;
    Ok(())
}
