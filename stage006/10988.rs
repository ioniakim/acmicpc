use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let word = input.trim().to_owned();

    let mut iter = word.chars();
    let mut rev = word.chars().rev();
    for _ in 0..word.len() / 2 {
        if iter.next().unwrap() != rev.next().unwrap() {
            println!("0");
            return Ok(())
        }
    }
    println!("1");
    Ok(())
}
