const DIFF: u16 = 2541 - 1998;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let buda_year = input.trim().parse::<u16>()?;

    println!("{}", buda_year - DIFF);

    Ok(())
}
