use std::io;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let word_bytes = input.trim().as_bytes();

    let mut count = 0;

    let mut pos = 0;
    while pos < word_bytes.len() {
        let word_left = &word_bytes[pos..];
        if word_left.len() > 1 {
            pos += next_char(word_left);
        }
        count += 1;
        pos += 1;
    }

    println!("{count}");
    Ok(())
}

fn next_char(word: &[u8]) -> usize {
    match word[0] as char {
        'c' => c_start(&word),
        'd' => d_start(&word),
        'l' => l_start(&word),
        'n' => n_start(&word),
        's' => s_start(&word),
        'z' => z_start(&word),
        _ => 0,
    }
}

fn c_start(word_bytes: &[u8]) -> usize {
    match word_bytes[1] as char {
        '=' | '-'  => 1,
        _ => 0,
    }
}

fn d_start(word_bytes: &[u8]) -> usize {
    match word_bytes[1] as char {
        '-' => 1,
        'z' => {
            if word_bytes.len() > 2 && word_bytes[2] as char == '=' { return 2 } else { 0 }
        }
        _ => 0,
    }
}

fn l_start(word_bytes: &[u8]) -> usize {
    match word_bytes[1] as char {
        'j' => 1,
        _ => 0,
    }
}

fn n_start(word_bytes: &[u8]) -> usize {
    match word_bytes[1] as char {
        'j' => 1,
        _ => 0,
    }
}

fn s_start(word_bytes: &[u8]) -> usize {
    match word_bytes[1] as char {
        '=' => 1,
        _ => 0,
    }
}

fn z_start(word_bytes: &[u8]) -> usize {
    match word_bytes[1] as char {
        '=' => 1,
        _ => 0,
    }
}
