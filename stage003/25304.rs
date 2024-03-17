use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let total_sum: u32 = input.trim().parse()?;

    input.clear();
    io::stdin().read_line(&mut input)?;
    let count: u32 = input.trim().parse()?;

    let mut result: u32 = 0;
    for _ in 0..count {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let item: Vec<u32> = input.split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        if item.len() != 2{
            return Err("Invalid item info".into());
        }

        let (price, num) = (item[0], item[1]);

        result += price * num;
    }

    let answer = if result == total_sum { "Yes" } else { "No" };

    println!("{answer}");
    Ok(())
}
