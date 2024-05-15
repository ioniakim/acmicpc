use std::io::{stdin, read_to_string};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(stdin())?;
    let mut iter = input.split_whitespace();
    let mut next = || iter.next().ok_or("No More Input".to_owned());

    let a = Fraction{numerator: next()?.parse()?, denomiator: next()?.parse()?};
    let b = Fraction{numerator: next()?.parse()?, denomiator: next()?.parse()?};

    let answer = solve(&a, &b);
    println!("{} {}", answer.numerator, answer.denomiator);
    Ok(())
}

struct Fraction {
    numerator: u64,
    denomiator: u64,
}

fn solve(frac1: &Fraction, frac2: &Fraction) -> Fraction {
    let mut common_denomiator = frac1.denomiator;
    let mut new_numerator = frac1.numerator + frac2.numerator;

    if frac1.denomiator != frac2.denomiator {
        common_denomiator = frac1.denomiator * frac2.denomiator;
        new_numerator = (frac1.numerator * frac2.denomiator) + (frac2.numerator * frac1.denomiator);
    }

    let gcf = gcf(new_numerator, common_denomiator);
    Fraction{numerator: new_numerator / gcf, denomiator: common_denomiator / gcf}
}

fn gcf(num1: u64, num2: u64) -> u64 {
    match num1 % num2 {
        0 => num2,
        remainder => gcf(num2, remainder),
    }
}
