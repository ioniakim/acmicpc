use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse()?;

    let mut result = 0;
    let mut creator = n - 1;
    while creator >= 1 {
        let mut sum = creator;
        let mut m = creator;
        while m > 0 {
            let digit = m % 10;
            sum += digit;
            m = m / 10;
        }

        if sum == n {
            result = creator;
        }

        creator -= 1;
    }

    println!("{result}");
    Ok(())
}
