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
        windows.iter_mut().enumerate()
            .filter(|(j, _)| j % i == 0)
            .for_each(|(_, w)| *w = !*w)
    }

    windows[1..].iter()
        .filter(|&&w| w)
        .count()
}
