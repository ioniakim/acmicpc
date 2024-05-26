/**
 * Given a natural number (0 <= n <= 4*n^9), find the least prime number greater than or equal to n.
 *
 *
 * */
use std::io::prelude::*;
use std::io::{stdin, stdout, BufWriter};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    let mut stdin = stdin().lock();
    stdin.read_line(&mut input)?;
    let n: usize = input.trim().parse()?;

    let mut out = BufWriter::new(stdout());
    stdin.lines().take(n)
        .flatten()
        .flat_map(|s| s.parse::<u64>())
        .map(|num| next_prime(num))
        .for_each(|prime| writeln!(out, "{prime}").unwrap());
    Ok(())
}

fn next_prime(num: u64) -> u64 {
    let mut num = match num {
        0 | 1 | 2 => return 2,
        num if num % 2 == 0 => num + 1,
        num => num,
    };

    while !is_prime(num) {
        num += 2;
    }

    num
}

fn is_prime(num: u64) -> bool {
    match num {
        0 | 1 => false,
        2 => true,
        num if num % 2 == 0 => false,
        _ => {
            let mut divisor = 3;
            loop {
                if num != divisor && num % divisor == 0 {
                    return false;
                }
                divisor += 2;
                if divisor * divisor > num {
                    return true
                }
            }
        }
    }
}
