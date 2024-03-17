use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let n: u16 = input.trim().parse()?;
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let nums: Vec<u16> = input.split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        println!("{}", nums[0] + nums[1]);
    }

    Ok(())
}
