use std::io;

const CHAR_SIZE: usize = ('z' as u8 - 'a' as u8 + 1) as usize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let count: usize = io::stdin().lines().skip(1)
        .flatten()
        .map(|line| {
            is_group_word(line.trim())
        })
        .map(|b| if b { 1 } else { 0 })
        .sum();

    println!("{count}");

    Ok(())
}

fn is_group_word(word: &str) -> bool {
    let mut appeared = [false; CHAR_SIZE];

    let mut prev = usize::MAX;
    for c in word.bytes() {
        let c = (c - 'a' as u8) as usize;
        if prev == usize::MAX {
            prev = c;
        }
        if appeared[c] {
            return false;
        } else if prev != c {
            appeared[prev] = true;
            prev = c;
        }
    }

    true
}
