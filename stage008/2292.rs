/// Honeycomb
/// 1 -> 7 -> 19 -> 37 -> 61
/// 1 -> 1 + 6 -> 7 + 12 -> 19 + 18 -> 37 + 24
/// next = prev + (6 * i)
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let room = input.trim().parse::<u32>()?;
    let mut layer = 0;
    let mut current = 1;

    loop {
        current = current + (6 * layer);
        layer += 1;
        if current >= room {
            println!("{layer}");
            break;
        }
    }

    Ok(())
}
