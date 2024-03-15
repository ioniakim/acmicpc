fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input)?;

    let numbers: Vec<u64> = input.split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    println!("{}", numbers[0] + numbers[1] + numbers[2]);


    Ok(())
}
