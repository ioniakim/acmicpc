use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::BufWriter::new(io::stdout());

    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                let sum = input.split_whitespace()
                    .map(str::parse::<u8>)
                    .sum::<Result<u8, _>>()?;
                input.clear();

                writeln!(out, "{}", sum)?;
            },
            Err(_) => break,
        }
    }
    Ok(())
}
