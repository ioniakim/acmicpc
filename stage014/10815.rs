use std::io::{stdin, stdout, Write, BufWriter};

static mut HOLD: [u8; 20_000_002] = [0u8; 20_000_002];
const BASE: i32 = 10_000_000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = BufWriter::new(stdout());
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>()?;
    input.clear();
    stdin().read_line(&mut input)?;
    for num in input.split_whitespace().take(n) {
        let card = num.trim().parse::<i32>()?;
        unsafe {
            HOLD[(card + BASE) as usize] = 1;
        }
    }

    input.clear();
    stdin().read_line(&mut input)?;
    let m = input.trim().parse::<usize>()?;
    input.clear();
    stdin().read_line(&mut input)?;
    for num in input.split_whitespace().take(m) {
        let card = num.trim().parse::<i32>()?;
        unsafe {
            write!(out, "{} ", HOLD[(card + BASE) as usize])?;
        }
    }

    writeln!(out)?;
    Ok(())
}

