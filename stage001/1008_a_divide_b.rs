use std::io;

fn main()
-> Result<(), Box<dyn std::error::Error>> {

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let nums = input.split_whitespace()
        .map(|s| { s.parse::<f64>() })
        .collect::<Result<Vec<_>, _>>()?;

    if nums.len() != 2 {
        return Err("Invalid input: invalid number of arguments".into());
    }

    println!("{}", nums[0] / nums[1]);
    Ok(())
}
