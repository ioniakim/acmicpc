use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let n: u16 = input.trim().parse()?;
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let sum: u16 = input.split_whitespace()
            .take(2)
            .map(str::parse::<u16>)
            .sum::<Result<u16, _>>()?;

        println!("{}", sum);
    }

    Ok(())
}
