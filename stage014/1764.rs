use std::collections::BTreeSet;
use std::io::{stdin, read_to_string, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(BufReader::new(stdin()))?;

    let mut iter = input.split_whitespace();
    let mut next = || iter.next().ok_or::<String>("End of input".into());
    let n = next()?.parse::<usize>()?;
    let m = next()?.parse::<usize>()?;

    let not_heard = (0..n).map(|_| next()).collect::<Result<BTreeSet<_>, _>>()?;
    let not_seen = (0..m).map(|_| next()).collect::<Result<BTreeSet<_>, _>>()?;

    let both = not_heard.intersection(&not_seen).map(|s| *s).collect::<Vec<_>>();

    println!("{}", both.len());
    println!("{}", both.join("\n"));

    Ok(())
}
