use std::io::prelude::*;
use std::io::{stdin, stdout, BufWriter};

fn main() {
    let stdin = stdin().lock();
    let mut out = BufWriter::new(stdout());
    stdin.lines().take_while(|v| {
            v.as_ref().unwrap() != "0"
        })
        .flatten()
        .flat_map(|s| s.parse::<usize>())
        .map(solve)
        .for_each(|count| writeln!(out, "{count}").unwrap());
}

fn solve(num: usize) -> usize {
    let upper = num * 2;
    let mut is_primes = vec![true; upper + 1];
    is_primes[0] = false;
    is_primes[1] = false;

    for i in 2..=upper {
        if !is_primes[i] {
            continue;
        }
        for j in (i..=upper).step_by(i) {
            if j != i {
                is_primes[j] = false;
            }
        }
    }

    is_primes[num+1..].into_iter()
        .filter(|&&is_prime| is_prime)
        .count()
}
