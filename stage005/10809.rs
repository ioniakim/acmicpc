use std::io;
use io::Write;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::BufWriter::new(io::stdout());
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut char_position: HashMap<char, usize> = HashMap::new();
    input.trim().chars()
        .enumerate()
        .for_each(|(i, c)| {
            char_position.entry(c).or_insert(i);
        });

    for c in 'a'..='z' {
        match char_position.get(&c) {
            Some(i) => write!(out, "{i} ")?,
            None => write!(out, "-1 ")?,
        }
    }

    Ok(())
}
