use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let values: Vec<u16> = input.split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let n = values[0];
    let k = values[1];

    let mut count = 0;
    let mut factor = 0;

    for i in 1..=n {
        if n % i == 0 {
            count += 1;
        }
        if count == k {
            factor = i;
            break;
        }
    }
    println!("{factor}");
    Ok(())
}
