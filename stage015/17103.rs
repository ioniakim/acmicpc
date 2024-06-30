use std::io::{stdin, read_to_string, stdout, BufWriter};
use std::io::prelude::*;

struct Eratosthenes {
    sieve: Vec<bool>
}

impl Eratosthenes {

    fn with_size(size: usize) -> Self {
        let mut sieve = vec![true; size];

        Self::sieve(&mut sieve);

        Eratosthenes { sieve: sieve }
    }

    fn sieve(sieve: &mut Vec<bool>) {
        (sieve[0], sieve[1]) = (false, false);
        let len = sieve.len();
        for i in (2..).take_while(|i| i * i <= len) {
            if !sieve[i] { continue; }
            for j in (i * i..len).step_by(i) {
                sieve[j] = false;
            }
        }
    }
}


/**
 * 1. Find all primes under a given number
 * 2. For each pair of two prime numbers including the same prime number,
 * count when the sum of each pair is the same as the given number.
 */
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = stdin().lock();

    let input_nums = read_to_string(stdin)?.split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()?;

    let eratos = Eratosthenes::with_size(*input_nums.iter().max().ok_or("No Value")?);

    let mut out = BufWriter::new(stdout());
    for num in input_nums {
        let count = eratos.sieve[..=num/2].iter().enumerate()
            .filter(|(_, &b)| b)
            .filter(|(i, _)| eratos.sieve[num - i])
            .count();
        writeln!(out, "{count}")?;
    }

    Ok(())
}

#[allow(dead_code)]
fn binary_search(nums: &[usize], target: usize) -> usize {
    if nums.len() == 0 {
        return 0;
    }
    let mid = nums.len() / 2;
    if nums[mid] < target {
        mid + binary_search(&nums[mid+1..], target) + 1
    } else if nums[mid] > target {
        binary_search(&nums[..mid], target)
    } else {
        mid
    }
}
