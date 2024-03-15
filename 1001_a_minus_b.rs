use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str)?;

    let numbers = input_str.split_whitespace()
        .map(|s| { s.parse::<i8>() })
        .collect::<Result<Vec<_>, _>>()?;

    if numbers.len() != 2 {
        return Err("Incrrect numbers of inputs".into());
    }

    println!("{}", numbers[0] - numbers[1]);

    Ok(())
}
