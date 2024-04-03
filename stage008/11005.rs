use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let values: Vec<u32> = input.split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let mut num: u32 = values[0];
    let num_system: u32 = values[1];

    let mut result = String::new();
    while num > 0 {
        let digit = num_to_digit_char(num % num_system)?;
        result.push(digit);
        num /= num_system;
    }
    result = result.chars().rev().collect();
    println!("{result}");
    Ok(())
}

fn num_to_digit_char(num: u32) -> Result<char, String> {
    match num {
        0..=9 => Ok(('0' as u8 + num as u8) as char),
        10..=35 => Ok(('A' as u8 + num as u8 - 10u8) as char),
        _ => Err("Wrong number".into()),
    }
}
