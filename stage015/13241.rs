fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let nums = input.trim().split_whitespace()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()?;

    let lcm = solve(nums[0], nums[1]);
    println!("{lcm}");
    Ok(())
}

fn solve(num1: u64, num2: u64) -> u64 {
    num1 * num2 / gcf(num1, num2)
}

fn gcf(num1: u64, num2: u64) -> u64 {
    match num1 % num2 {
        0 => num2,
        remainder => gcf(num2, remainder)
    }
}
