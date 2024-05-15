use std::io::{stdin, BufReader, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = BufReader::new(stdin());

    let mut input = String::new();
    reader.read_line(&mut input)?;
    let n: usize = input.trim().parse()?;

    for line in reader.lines().take(n) {
        let line = line?;
        let mut iter = line.split_whitespace()
            .flat_map(str::parse::<usize>);
        let num1 = iter.next().ok_or("".to_owned())?;
        let num2 = iter.next().ok_or("".to_owned())?;

        let lcm = solve(num1, num2);

        println!("{lcm}");
    }

    Ok(())
}

fn solve(num1: usize, num2: usize) -> usize {
    match gcf(num1, num2) {
        1 => num1 * num2,
        gcf if gcf == num1 => num2,
        gcf if gcf == num2 => num1,
        _ => {
            let base = std::cmp::max(num1, num2);
            let other = std::cmp::min(num1, num2);
            let mut i = 2;
            loop {
                let multiple = base * i;
                if multiple % other == 0 {
                    break multiple;
                }
                i += 1;
            }
        },
    }
}

fn gcf(num1: usize, num2: usize) -> usize {
    if num1 < num2 {
        return gcf(num2, num1);
    }

    match num1 % num2 {
        0 => num2,
        remainder => gcf(num2, remainder),
    }
}
