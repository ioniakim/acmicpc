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
        EratosthenesIter::new(2, self.sieve.len(), &self.sieve[..])
    }

    fn range(&self, front: usize, end: usize) -> EratosthenesIter {
        EratosthenesIter::new(front, end, &self.sieve[..])
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
    front: usize,
    back: usize,
    sieve: &'a [bool]
}

impl<'a> EratosthenesIter<'a> {
    fn new(front: usize, back: usize, sieve: &'a [bool] ) -> Self {
        EratosthenesIter {front: front, back: back, sieve: sieve}
    }
}

impl<'a> Iterator for EratosthenesIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.front;
        while next < self.back && !self.sieve[next] {
            next += 1;
        }
        if next < self.back {
            self.front = next + 1;
            Some(next)
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for EratosthenesIter<'a> {

    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back < 2 {
            return None;
        }
        let mut next_back = self.back - 1;
        while next_back >= self.front && !self.sieve[next_back] {
            next_back -= 1;
        }

        if next_back >= self.front {
            self.back = next_back;
            Some(next_back)
        } else {
            None
        }
    }
}

/**
 * 1. Find all primes under a given number
 * 2. For each pair of two prime numbers including the same prime number,
 * count when the sum of each pair is the same as the given number.
 */
fn main() {
    let eratos = Eratosthenes::with_size(12);

    for p in eratos.range(3, 7).rev() {
        println!("{p}");
    }
    // let n = 100;
    // let mut count = 0;
    // for p1 in eratos.iter().take_while(|&p| p <= n / 2) {
    //     for p2 in eratos.iter().rev().take_while(|&p| p >= n / 2) {
    //         if p1 + p2 == n {
    //             count += 1;
    //             println!("{p1} + {p2} = {}", p1 + p2);
    //         }
    //     }
    // }
    // println!("{count}");
}
