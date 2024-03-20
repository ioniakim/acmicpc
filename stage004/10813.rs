use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::BufWriter::new(io::stdout());

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let first_line: Vec<usize> = input.split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let n = first_line[0];
    let m = first_line[1];

    let mut baskets: Vec<String> = Vec::with_capacity(n);
    (0..n).for_each(|i| baskets.push((i + 1).to_string()));

    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let switching_baskets: Vec<usize> = input.split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        baskets.swap(switching_baskets[0] - 1, switching_baskets[1] - 1);
    }

    writeln!(out, "{}", baskets.join(" "))?;
    Ok(())
}
