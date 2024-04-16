use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut primes = [true; 1001];
    set_primes(&mut primes);

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>()?;

    input.clear();
    io::stdin().read_line(&mut input)?;
    let numbers: Vec<usize> = input.split_whitespace()
        .take(n)
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let count = numbers.iter()
        .filter(|&&n| primes[n])
        .count();

    println!("{count}");
    Ok(())
}

fn set_primes(primes: &mut [bool]) {
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
