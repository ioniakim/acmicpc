/// faction
/// 1/1, 1/2, 2/1, 3/1, 2/2, 1/3, 1/4, 2/3, 3/2, 4/1, ...
///
/// seed = 1..
/// numerator = 1..=seed
/// denominator = (1..=seed).rev()
/// count += 1

use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let x = input.trim().parse::<u32>()?;

    let mut seed = 1;
    let mut count = 0;

    'outer: loop {
        for (numerator, denominator) in get_iterator(seed) {
            count += 1;
            if count == x {
                println!("{numerator}/{denominator}");
                break 'outer;
            }
        }
        seed += 1;
    }

    Ok(())
}

fn get_iterator(seed: u32) -> Box<dyn std::iter::Iterator<Item = (u32, u32)>> {
    if seed % 2 == 0 {
        Box::new((1..=seed).zip((1..=seed).rev()))
    } else {
        Box::new((1..=seed).rev().zip(1..=seed))
    }
}
