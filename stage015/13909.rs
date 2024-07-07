/**
 * Failed for now. Try it again later
 */
use std::io::{stdin};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>()?;
    let count = solve(n);
    println!("{count}");
    Ok(())
}

/**
 * All prime numbers are open
 *  1 2 3 4 5 6 7 8 9 10
 *1 1 1 1 1 1 1 1 1 1  1
 *2 1 0 1 0 1 0 1 0 1  0
 *3 1 0 0 0 1 1 1 0 0  0
 *4 1 0 0 1 1 1 1 1 0  0
 *5 1 0 0 1 0 1 1 1 0  1
 */
fn solve(n: usize) -> usize {
    let n = n + 1;
    let mut windows = vec![true; n];

    for i in (2..).take_while(|i| i * i < n){
        if !windows[i] { continue; }
        for j in (i * i..n).step_by(i){
            windows[j] = false;
        }
    }

    windows[1..].iter()
        .filter(|&&w| w)
        .count()
}
