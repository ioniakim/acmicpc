use std::io::{stdin, read_to_string};
use std::ops::Add;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(stdin())?;
    let mut iter = input.split_whitespace();
    let mut next = || iter.next().ok_or("No More Input".to_owned());

    let a = Fraction{numerator: next()?.parse()?, denomiator: next()?.parse()?};
    let b = Fraction{numerator: next()?.parse()?, denomiator: next()?.parse()?};

    let answer = (a + b).reduce();
    println!("{} {}", answer.numerator, answer.denomiator);
    Ok(())
}

#[derive(Copy, Clone, PartialEq)]
struct Fraction {
    numerator: u64,
    denomiator: u64,
}

impl Fraction {
    pub fn reduce(self) -> Self {
        let gcf = gcf(self.denomiator, self.numerator);
        Fraction {
            denomiator: self.denomiator / gcf,
            numerator: self.numerator / gcf,
        }
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        if self.denomiator == rhs.denomiator {
            Self {
                denomiator: self.denomiator,
                numerator: self.numerator + rhs.numerator,
            }
        } else {
            Self {
                denomiator: self.denomiator * rhs.denomiator,
                numerator: (self.numerator * rhs.denomiator) + (rhs.numerator * self.denomiator)
             }
        }
    }
}

fn gcf(num1: u64, num2: u64) -> u64 {
    match num1 % num2 {
        0 => num2,
        remainder => gcf(num2, remainder),
    }
}
