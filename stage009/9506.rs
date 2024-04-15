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
    let mut factors = vec![];
    let mut sum = 0;
    for i in 1..n {
        if n % i == 0 {
            factors.push(i);
            sum += i;
        }
    }

    if sum == n {
        let rh = factors.iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(" + ");
        println!("{n} = {rh}");
    } else {
        println!("{n} is NOT perfect.");
    }
}
