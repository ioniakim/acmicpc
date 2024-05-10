use std::collections::BTreeSet;
use std::io::{stdin, stdout, Write, BufWriter};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>()?;
    let mut entry_register = BTreeSet::new();

    for line in stdin().lines().take(n) {
        let line = line?;
        let mut record = line.split_whitespace();
        let name = record.next().ok_or::<String>("No Name".into())?;
        let status = record.next().ok_or::<String>("No Status".into())?;
        match status {
            "enter" => entry_register.insert(name.to_owned()),
            _ => entry_register.remove(name),
        };
    }
    let mut out = BufWriter::new(stdout());
    for name in entry_register.iter().rev() {
        writeln!(out, "{name}")?;
    }
    Ok(())
}
