fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input)?;

    let score: u8 = input.trim().parse()?;

    if score > 100 {
        return Err("The score is over 100".into());
    }

    let grade = match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    };
    println!("{grade}");

    Ok(())
}
