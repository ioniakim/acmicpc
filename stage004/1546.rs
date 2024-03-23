use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let n: usize = input.trim().parse()?;

    let mut max: u8 = u8::MIN;
    input.clear();
    io::stdin().read_line(&mut input)?;
    let scores: Vec<u8> = input.split_whitespace()
        .take(n)
        .flat_map(str::parse)
        .inspect(|&score| {
            max = std::cmp::max(max, score);
        })
        .collect();

    let mean = scores.into_iter()
        .map(|score| {
            score as f32 / max as f32 * 100.
        })
        .sum::<f32>() / n as f32;

    println!("{}", mean);
    Ok(())
}
