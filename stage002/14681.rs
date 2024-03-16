use std::io::stdin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let x: i16 = input.trim().parse()?;
    input.clear();
    stdin().read_line(&mut input)?;
    let y: i16 = input.trim().parse()?;

    let quadrant = read_quadrant(x, y)?;
    println!("{quadrant}");

    Ok(())
}

fn read_quadrant(x: i16, y: i16) -> Result<u8, String> {
    match (x, y) {
        (x, y) if x > 0 && y > 0 => Ok(1),
        (x, y) if x > 0 && y < 0 => Ok(4),
        (x, y) if x < 0 && y > 0 => Ok(2),
        (x, y) if x < 0 && y < 0 => Ok(3),
        _ => Err("Invalid Coordinates".into()),
    }
}
