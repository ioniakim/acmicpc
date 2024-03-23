use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let nums: Vec<usize> = input.split_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    println!("{}", std::cmp::max(rev_num(nums[0]), rev_num(nums[1])));

    Ok(())
}

#[allow(dead_code)]
fn rev_num(mut num: usize) -> usize {
    let mut rev = 0usize;
    while num > 0 {
        rev *= 10;
        rev += num % 10;
        num /= 10;
    }

    rev
}
