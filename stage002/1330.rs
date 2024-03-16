fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut input = String::new();

    std::io::stdin().read_line(&mut input)?;

    let numbers: Vec<i32> = input.split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    if numbers.len() != 2 {
        return Err("Invalid number of inputs".into());
    }

    if numbers[0] < numbers[1] {
        println!("<");
    } else if numbers[0] > numbers[1] {
        println!(">");
    } else {
        println!("==");
    }
    Ok(())
}
