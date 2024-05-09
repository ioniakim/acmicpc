use std::io::Write;

static mut HOLD: [u8; 20_000_002] = [0u8; 20_000_002];
const BASE: i32 = 10_000_000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>()?;
    input.clear();
    std::io::stdin().read_line(&mut input)?;
    let sanguns = input.split_whitespace().take(n)
        .map(str::parse::<i32>)
        .collect::<Result<Vec<_>, _>>()?;

    input.clear();
    std::io::stdin().read_line(&mut input)?;
    let m = input.trim().parse::<usize>()?;
    input.clear();
    std::io::stdin().read_line(&mut input)?;
    let cards = input.split_whitespace().take(m)
        .map(str::parse::<i32>)
        .collect::<Result<Vec<_>, _>>()?;

    let mut out = std::io::BufWriter::new(std::io::stdout());
    unsafe {
        sanguns.iter().for_each(|c| HOLD[(c + BASE) as usize] = 1);

        for c in &cards {
            write!(out, "{} ", HOLD[(c + BASE) as usize])?;
        }
    }

    writeln!(out)?;
    Ok(())
}

