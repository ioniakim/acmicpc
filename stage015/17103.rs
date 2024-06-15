use std::io::{stdin, read_to_string, stdout, BufWriter};
use std::io::prelude::*;
use std::ops::Range;

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

    fn iter(&self) -> EratosthenesIter {
        EratosthenesIter::new(&self.sieve[..], 2..self.sieve.len())
    }

    #[allow(dead_code)]
    fn range(&self, range: Range<usize>) -> EratosthenesIter {
        EratosthenesIter::new(&self.sieve[..], range)
    }

    fn range_from(&self, from: usize) -> EratosthenesIter {
        let range = Range{start: from, end: self.sieve.len()};
        EratosthenesIter::new(&self.sieve[..], range)
    }

    fn range_to(&self, to: usize) -> EratosthenesIter {
        let range = Range{start: 2, end: to};
        EratosthenesIter::new(&self.sieve[..], range)
    }
}


impl<'a> IntoIterator for &'a Eratosthenes {
    type Item = usize;
    type IntoIter = EratosthenesIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

struct EratosthenesIter<'a> {
    sieve: &'a [bool],
    range: Range<usize>,
}

impl<'a> EratosthenesIter<'a> {
    fn new(sieve: &'a [bool], range: Range<usize>) -> Self {
        EratosthenesIter {sieve: sieve, range: range}
    }
}

impl<'a> Iterator for EratosthenesIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.range.next() {
            if self.sieve[next] {
                return Some(next)
            }
        }
        None
    }
}

impl<'a> DoubleEndedIterator for EratosthenesIter<'a> {

    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(next_back) = self.range.next() {
            if self.sieve[next_back] {
                return Some(next_back);
            }
        }
        None
    }
}

struct GoldbachPartition {
    sieve: Eratosthenes,
}

impl GoldbachPartition {
    fn with_size(size: usize) -> Self {
        GoldbachPartition { sieve: Eratosthenes::with_size(size) }
    }

    fn solve(&self, num: usize) -> usize {
        let mut count = 0;
        for p1 in self.sieve.range_to(num/2 + 1) {
            for p2 in self.sieve.range_from(num/2).rev() {
                if p1 + p2 == num {
                    count += 1;
                }
            }
        }
        count
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

    let goldbach = GoldbachPartition::with_size(*input_nums.iter().max().ok_or("No Max Value")?);
    let mut out = BufWriter::new(stdout());
    for num in input_nums {
        writeln!(out, "{}", goldbach.solve(num))?;
    }
    Ok(())
}
