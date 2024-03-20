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

    let mut baskets = vec![0u8; n];

    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let method: Vec<u8> = input.split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        for i in (method[0] - 1)..=(method[1] - 1) {
            baskets[i as usize] = method[2];
        }
    }

    writeln!(out, "{}", baskets.iter().map(u8::to_string).collect::<Vec<_>>().join(" "))?;

    Ok(())
}
