use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let word = input.trim().to_owned();

    let forward = word.chars();
    let backward = word.chars().rev();
    match forward.zip(backward).all(|(f1, b2)| f1 == b2) {
        true => println!("1"),
        false => println!("0"),
    }
    Ok(())
}
