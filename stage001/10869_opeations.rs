fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let nums = input.split_whitespace()
        .map(|s| { s.parse::<i32>() })
        .collect::<Result<Vec<_>, _>>()?;

    if nums.len() != 2 {
        return Err("Invalid inputs: invalid number of inputs".into());
    }

    println!("{}", nums[0] + nums[1]);
    println!("{}", nums[0] - nums[1]);
    println!("{}", nums[0] * nums[1]);
    println!("{}", nums[0] / nums[1]);
    println!("{}", nums[0] % nums[1]);

    Ok(())
}
