use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse()?;

    let mut result = 0;
    for creator in 1..n {
        let mut sum = creator;
        let mut m = creator;
        while m > 0 {
            sum += m % 10;
            m = m / 10;
        }

        if sum == n {
            result = creator;
            break;
        }
    }

    println!("{result}");
    Ok(())
}
