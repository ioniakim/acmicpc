use std::io::{
    stdin,
    stdout,
    read_to_string,
    BufWriter,
    Write
};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(stdin())?;
    let mut iter = input.split_ascii_whitespace();
    let mut next = || iter.next().ok_or::<String>("No More Input".into());

    let n: usize = next()?.parse()?;
    let m: usize = next()?.parse()?;

    let poketmons = (0..n).map(|_| next()).collect::<Result<Vec<_>, _>>()?;
    let name_map = poketmons.iter().enumerate()
        .map(|(no, name)| (name, no + 1))
        .collect::<HashMap<&&str, usize>>();

    let mut out = BufWriter::new(stdout());
    for _ in 0..m {
        let input = next()?;
        match input.parse::<usize>() {
            Ok(no) => writeln!(out, "{}", poketmons[no - 1])?,
            _ => writeln!(out, "{}", name_map.get(&input).ok_or::<String>("No matching name".into())?)?,
        }
    }

    Ok(())
}
