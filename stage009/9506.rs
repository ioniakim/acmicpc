use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    for line in io::stdin().lines() {
        let line = line?;
        if line == "-1".to_string() {
            break;
        }
        let n: u32 = line.parse()?;
        solve(n as u32);
    }
    Ok(())
}

fn solve(n: u32) {
    let factors: Vec<u32> = (1..n).filter(|v| n % v == 0).collect();

    if n == factors.iter().sum() {
        let rh = factors.iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(" + ");
        println!("{n} = {rh}");
    } else {
        println!("{n} is NOT perfect.");
    }
}
