use std::io::{stdin};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>()?;
    let count = solve(n);
    println!("{count}");
    Ok(())
}

fn solve(n: usize) -> usize {
    let mut windows = vec![true; n + 1];

    for i in 2..n+1 {
        for (j, w) in windows.iter_mut().enumerate() {
            if j % i == 0 {
                *w = !*w;
            }
        }
    }

    windows[1..].iter()
        .filter(|&&w| w)
        .count()
}
