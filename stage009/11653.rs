use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut n = input.trim().parse::<usize>()?;

    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            n /= i;
            println!("{i}");
        }

        i += 1;
    }
    if n != 1 {
        println!("{n}");
    }
    Ok(())
}
