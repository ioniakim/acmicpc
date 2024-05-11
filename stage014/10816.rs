use std::io::{stdin, read_to_string, BufReader};

fn main()->Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(BufReader::new(stdin()))?;
    let mut iter = input.split_whitespace();
    let mut next = || iter.next().ok_or::<String>("End of input".into());

    let n = next()?.parse::<usize>()?;

    let mut card_counter = CardCounter::new();
    (0..n).map(|_| next()).flatten().flat_map(str::parse::<isize>).for_each(|num| card_counter.count(num));

    let m = next()?.parse::<usize>()?;
    let result = (0..m).map(|_| next()).flatten()
        .flat_map(str::parse::<isize>)
        .map(|num| card_counter.get_count(num))
        .map(|num| num.to_string())
        .collect::<Vec<_>>().join(" ");

    println!("{result}");
    Ok(())
}

struct CardCounter {
    count: Vec<usize>,
}

impl CardCounter {
    const BASE: isize = 10_000_000;
    fn new() -> Self {
        CardCounter {count: vec![0; 20_000_002]}
    }

    fn count(&mut self, num: isize) {
        self.count[(num + Self::BASE) as usize] += 1;
    }

    fn get_count(&self, num: isize) -> usize {
        self.count[(num + Self::BASE) as usize]
    }
}
