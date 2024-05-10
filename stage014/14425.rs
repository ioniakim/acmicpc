use std::collections::HashSet;
use std::io::{stdin};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let mut iter = input.split_whitespace().take(2)
        .flat_map(str::parse::<usize>);
    let n = iter.next().ok_or::<String>("No N".into())?;
    let m = iter.next().ok_or::<String>("No M".into())?;

    let strings = stdin().lines().take(n)
        .flatten()
        .collect::<HashSet<String>>();

    let count = stdin().lines().take(m)
        .flatten()
        .filter(|s| strings.contains(s))
        .count();

    println!("{count}");

    Ok(())
}
