use std::collections::BTreeSet;
use std::io::{stdin, read_to_string};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let input = read_to_string(stdin())?;

    let mut iter = input.split_whitespace();
    let mut next = || iter.next().ok_or::<String>("No More".into());

    let n: usize = next()?.parse()?;
    let mut entry_register = BTreeSet::new();
    for _ in 0..n {
        let name = next()?;
        let status = next()?;
        match status {
            "enter" => entry_register.insert(name.to_owned()),
            _ => entry_register.remove(name),
        };
    }

    let result = entry_register.into_iter().rev().collect::<Vec<_>>().join("\n");
    println!("{result}");

    Ok(())
}
