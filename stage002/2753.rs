fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input)?;
    let year: u16 = input.trim().parse()?;

    if is_leap_year(year) {
        println!("1");
    } else {
        println!("0");
    }

    Ok(())
}

fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
