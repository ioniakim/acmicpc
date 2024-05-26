/**
 * Sieve of Eratosthenes
 */
use std::io::{stdin, stdout, BufWriter};
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let mut iter = input.split_whitespace().flat_map(str::parse::<usize>);
    let m = iter.next().ok_or("No Input".to_owned())?;
    let n = iter.next().ok_or("No Input".to_owned())?;
    let primes = solve(m, n);

    let mut out = BufWriter::new(stdout());

    for prime in primes {
        writeln!(out, "{prime}")?;
    }
    Ok(())
}

fn solve(min: usize, max: usize) -> Vec<usize> {
    if max < 2 {
        return vec![];
    }
    let mut is_primes = vec![true; max + 1];
    is_primes[0] = false;
    is_primes[1] = false;
    for i in 2..=max {
        if !is_primes[i] { continue; };
        for j in (i+i..=max).step_by(i) {
            is_primes[j] = false;
        }
    }

    is_primes[min..].into_iter().zip(min..)
        .filter(|(is_prime, _)| **is_prime)
        .map(|(_, i)| i)
        .collect()
}
