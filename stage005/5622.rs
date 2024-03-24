use std::io;

fn alpha_num(c: char) -> u16 {
    match c {
        'A'..='C' => 3,
        'D'..='F' => 4,
        'G'..='I' => 5,
        'J'..='L' => 6,
        'M'..='O' => 7,
        'P'..='S' => 8,
        'T'..='V' => 9,
        'W'..='Z' => 10,
        _ => 0,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let seconds = input.chars()
        .map(alpha_num)
        .sum::<u16>();

    println!("{seconds}");
    Ok(())
}
