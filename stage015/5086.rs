use std::io::{stdin, read_to_string, BufReader, stdout, Write, BufWriter};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(BufReader::new(stdin()))?;
    let mut iter = input.split_whitespace();
    let mut next = || iter.next().ok_or::<String>("End of input".into());

    let mut out = BufWriter::new(stdout());

    loop {
        let num1: usize = next()?.parse()?;
        let num2: usize = next()?.parse()?;

        if num1 == 0 && num2 == 0 {
            break;
        }

        writeln!(out, "{}", solve(num1, num2))?;
    }

    Ok(())
}

fn solve(num1: usize, num2: usize) -> &'static str{
    if num2 % num1 == 0 {
        return "factor";
    }

    if num1 % num2 == 0 {
        return "multiple";
    }

    "neither"
}
