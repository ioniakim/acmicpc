use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut primes = [true; 10_001];
    set_primes(&mut primes);

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let m = input.trim().parse::<usize>()?;

    input.clear();
    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>()?;

    let sum: usize = primes[m..=n].iter().enumerate()
        .map(|(i, b)| (i + m, b))
        .filter(|(_, b)| **b)
        .map(|(i, _)| i)
        .sum();
    let min: i32 = primes[m..=n].iter().enumerate()
        .map(|(i, b)| (i + m, b))
        .filter(|(_, b)| **b)
        .map(|(i, _)| i as i32)
        .min().unwrap_or(-1);

    if min > 0 {
        println!("{sum}");
    }
    println!("{min}");
    Ok(())
}

fn set_primes(primes: &mut [bool]) {
    primes[0] = false;
    primes[1] = false;

    for i in 2..primes.len() {
        if !primes[i] {
            continue;
        }
        let mut j = 2;
        let mut next = i * j;
        while next < primes.len() {
            if primes[next] {
                primes[next] = false;
            }
            j += 1;
            next = i * j;
        }
    }
}
