fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let first: u32 = input.trim().parse()?;

    input.clear();
    std::io::stdin().read_line(&mut input)?;
    let second = input.trim();

    let mut result = 0;
    let mut digit_place = 1;

    for c in second.chars().rev() {
        let d = c.to_digit(10)
            .ok_or("Invalid digit")?; // String implements the Error trait.
        let res = first * d;
        result += res * digit_place;
        digit_place *= 10;
        println!("{}", res);
    }

    println!("{}", result);
    Ok(())
}
