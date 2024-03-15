fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input)?;

    let nums = input.split_whitespace()
        .map(|s| s.parse::<u32>())
        .take(3)
        .collect::<Result<Vec<_>, _>>()?;

    if nums.len() < 3 {
        return Err("Invalid input: invalid number of arguments".into());
    }

    if nums.iter().any(|&n| n < 2 || n > 10000 ) {
        return Err("Invalid input: Numbers must be in the range [2, 10000]".into());
    }

    println!("{}", (nums[0] + nums[1]) % nums[2]);
    println!("{}", ((nums[0] % nums[2]) + (nums[1] % nums[2])) % nums[2]);
    println!("{}", (nums[0] * nums[1]) % nums[2]);
    println!("{}", ((nums[0] % nums[2]) * (nums[1] % nums[2])) % nums[2]);
    Ok(())
}
