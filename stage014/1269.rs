use std::collections::HashSet;
use std::io::{stdin, read_to_string, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(BufReader::new(stdin()))?;
    let mut iter = input.split_whitespace();
    let mut next = || iter.next().ok_or::<String>("No More Input".into());

    let n: usize = next()?.parse()?;
    let m: usize = next()?.parse()?;

    let a_set: HashSet<usize> = (0..n).map(|_| next()).flatten().flat_map(str::parse).collect();
    let b_set: HashSet<usize> = (0..m).map(|_| next()).flatten().flat_map(str::parse).collect();

    let a_count = a_set.difference(&b_set).count();
    let b_count = b_set.difference(&a_set).count();

    println!("{}", a_count + b_count);

    Ok(())
}
