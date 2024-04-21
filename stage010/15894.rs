/// n, n-1, n-2, ..., 2, 1
/// n == 1 => 1 + 3 = 4
/// n == 2 => 2 + 3 + 3 = 8
/// n == 3 => 3 + 3 + 3 + 3 = 12
/// n == i => i + sum_(j=0..i, 3)

use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse()?;

    let perimeter = n + (0..n).map(|_| 3).sum::<usize>();
    println!("{perimeter}");
    Ok(())
}
