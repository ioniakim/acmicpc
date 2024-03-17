use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut out = io::BufWriter::new(io::stdout().lock());

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let count: usize = input.trim().parse()?;

    for i in 1..=count {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let nums: Vec<u8> = input.split_whitespace()
            .take(2)
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        writeln!(out, "Case #{i}: {} + {} = {}", nums[0], nums[1], nums[0] + nums[1])?;
    }

    Ok(())
}
