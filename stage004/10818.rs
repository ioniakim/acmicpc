use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::BufWriter::new(io::stdout());

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let n: usize = input.trim().parse()?;

    input.clear();
    io::stdin().read_line(&mut input)?;
    let (min, max) = use_for(n, &input)?;

    writeln!(out, "{} {}", min, max)?;

    Ok(())
}

#[allow(dead_code)]
fn use_for(n: usize, input: &String) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    let mut min = i32::MAX;
    let mut max = i32::MIN;

    for result in input.split_whitespace().take(n).map(str::parse) {
        let result = result?;
        min = std::cmp::min(min, result);
        max = std::cmp::max(max, result);
    }

    Ok((min, max))
}

#[allow(dead_code)]
fn use_flat_map(n: usize, input: &String) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    input.split_whitespace()
        .take(n)
        .flat_map(str::parse)
        .for_each(|v| {
            min = std::cmp::min(min, v);
            max = std::cmp::max(max, v);
        });
    Ok((min, max))
}

#[allow(dead_code)]
fn use_map(n: usize, input: &String) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    input.split_whitespace()
        .take(n)
        .map(str::parse)
        .for_each(|v| {
            let v = v.unwrap();
            min = std::cmp::min(min, v);
            max = std::cmp::max(max, v);
        });
    Ok((min, max))
}
