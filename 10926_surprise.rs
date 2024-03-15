fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input)?;

    println!("{}??!", input.trim());
    Ok(())
}
