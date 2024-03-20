use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::BufWriter::new(io::stdout());

    let mut project = vec![false; 30];

    let mut input = String::new();
    for _ in 0..28 {
        io::stdin().read_line(&mut input)?;
        let submit: usize = input.trim().parse()?;
        input.clear();
        project[submit - 1] = true;
    }

    for (i, submit) in project.into_iter().enumerate() {
        if !submit {
            writeln!(out, "{}", i + 1)?;
        }
    }

    Ok(())
}
