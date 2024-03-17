use std::cmp;
use std::io;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let dices: Vec<u16> = input.split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    if dices.len() != 3 {
        return Err("Wrong number of dices".into());
    }

    let prize = match dices[..] {
        [ d1, d2, d3 ] if d1 == d2 && d2 == d3 => 10000 + d1 * 1000,
        [ d1, d2, d3 ] if d1 == d2 && d1 != d3 => 1000 + d1 * 100,
        [ d1, d2, d3 ] if d2 == d3 && d1 != d2 => 1000 + d2 * 100,
        [ d1, d2, d3 ] if d1 == d3 && d1 != d2 => 1000 + d3 * 100,
        [ d1, d2, d3 ] if d1 != d2 && d1 != d3 && d2 != d3 => cmp::max(cmp::max(d1, d2), d3) * 100,
        _ => 0,
    };

    println!("{prize}");

    Ok(())
}
