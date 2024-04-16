use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut n = input.trim().parse::<usize>()?;

    while n > 1 && !is_prime(n) {
        for i in 2..=n.div_ceil(2) {
            if !is_prime(i) { continue; }
            if n % i == 0 {
                println!("{i}");
                n /= i;
                break;
            }
        }
    }

    if n != 1 {
        println!("{n}");
    }
    Ok(())
}

fn is_prime(n: usize) -> bool {
    if n == 0 || n == 1 {
        return false;
    }

    if n == 2 || n == 3 {
        return true;
    }

    for i in 2..=n.div_ceil(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
