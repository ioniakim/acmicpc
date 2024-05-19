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
    match num1 % num2 {
        0 => num2,
        remainder => gcd(num2, remainder),
    }
}

fn solve(tree_set: &BTreeSet<usize>) -> Result<usize, Box<dyn std::error::Error>> {
    let mut tree_iter = tree_set.iter();
    let mut first = *(tree_iter.next().ok_or("".to_owned())?);
    let mut gaps: HashSet<usize> = tree_iter
        .map(|&p| {
            let gap = p - first;
            first = p;
            gap
        })
        .collect();

    while gaps.len() > 1 {
        let mut iter = gaps.into_iter();
        first = iter.next().ok_or("Invalid Value".to_owned())?;
        gaps = iter.map(|g| {
            let gcd = gcd(first, g);
            first = g;
            gcd
        })
        .collect();
    }

    let common_gap = *(gaps.iter().next().ok_or("Failed".to_owned())?);

    let last = *(tree_set.last().ok_or("No Value".to_owned())?);
    let first = *(tree_set.first().ok_or("No Value".to_owned())?);
    let count = (first..last).step_by(common_gap)
        .filter(|t| !tree_set.contains(t))
        .count();
    Ok(count)
}
