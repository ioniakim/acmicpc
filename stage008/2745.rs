use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let values = input.split_whitespace().collect::<Vec<&str>>();

    let number = values[0];
    let number_system = values[1].parse::<u32>()?;
    let mut decimal = 0;
    for (i, c) in number.chars().rev().enumerate() {
        let n = turn_to_number(c);
        decimal += n * number_system.pow(i as u32);
    }
    println!("{decimal}");
    Ok(())
}

fn turn_to_number(c: char) -> u32 {
    match c {
        '0'..='9' => c as u32 - '0' as u32,
        'A'..='Z' => 10 + c as u32 - 'A' as u32,
        _ => 0
    }
}
