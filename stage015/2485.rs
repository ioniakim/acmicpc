use std::collections::HashSet;
use std::collections::BTreeSet;
use std::io::{stdin};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse()?;
    let tree_set: BTreeSet<usize> = stdin().lines().take(n)
        .flatten()
        .map(|s| s.parse())
        .collect::<Result<BTreeSet<_>, _>>()?;

    let count = solve(&tree_set)?;
    println!("{count}");

    Ok(())
}

fn gcd(num1: usize, num2: usize) -> usize {
    if num1 < num2 {
        return gcd(num2, num1);
    }
    let mut num1 = num1;
    let mut num2 = num2;
    loop {
        match num1 % num2 {
            0 => break num2,
            remainder => {
                num1 = num2;
                num2 = remainder;
            },
        }
    }
}

fn solve(tree_set: &BTreeSet<usize>) -> Result<usize, Box<dyn std::error::Error>> {
    let mut iter = tree_set.iter();
    let mut prev = iter.next().ok_or("No Value".to_owned())?;

    let gaps: HashSet<usize> = iter.map(|t| {
        let gap = t - prev;
        prev = t;
        gap
    }).collect();

    let mut iter = gaps.iter();
    let mut common_gcd = *(iter.next().ok_or("No Gap".to_owned())?);
    common_gcd = iter.fold(common_gcd, |prev_gcd, &g| gcd(prev_gcd, g));

    let first = *(tree_set.first().ok_or("No First".to_owned())?);
    let last = *(tree_set.last().ok_or("No Last".to_owned())?);
    let count = (last - first) / common_gcd  - tree_set.len() + 1;

    Ok(count)
}
