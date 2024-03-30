use std::io;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let word = input.trim().as_bytes();
    println!("{}", word[0] as char);
    let mut iter = input.trim().chars();

    let mut count = 0;

    while let Some(c) = iter.next() {
        count += 1 + match c {
            'c' => c_start(&mut iter),
            'd' => d_start(&mut iter),
            'l' => l_start(&mut iter),
            'n' => n_start(&mut iter),
            's' => s_start(&mut iter),
            'z' => z_start(&mut iter),
            _ => 0
        }
    }
    println!("{count}");
    Ok(())
}

fn c_start(iter: &mut dyn std::iter::Iterator<Item = char>) -> usize {
    match iter.next() {
        Some(c) if c == '=' || c == '-'  => 0,
        _ => 1,
    }
}

fn d_start(iter: &mut dyn std::iter::Iterator<Item = char>) -> usize {
    match iter.next() {
        Some(c) if c == '-' => 0,
        Some(c) if c == 'z' => {
            match iter.next() {
                Some(c) if c == '=' => 0,
                _ => 2,
            }
        }
        _ => 1,
    }
}

fn l_start(iter: &mut dyn std::iter::Iterator<Item = char>) -> usize {
    match iter.next() {
        Some(c) if c == 'j' => 0,
        _ => 1,
    }
}

fn n_start(iter: &mut dyn std::iter::Iterator<Item = char>) -> usize {
    match iter.next() {
        Some(c) if c == 'j' => 0,
        _ => 1,
    }
}

fn s_start(iter: &mut dyn std::iter::Iterator<Item = char>) -> usize {
    match iter.next() {
        Some(c) if c == '=' => 0,
        _ => 1,
    }
}

fn z_start(iter: &mut dyn std::iter::Iterator<Item = char>) -> usize {
    match iter.next() {
        Some(c) if c == '=' => 0,
        _ => 1,
    }
}
