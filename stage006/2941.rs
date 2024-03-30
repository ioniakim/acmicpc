use std::io;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let word_bytes = input.trim().as_bytes();

    let mut count = 0;

    let mut pos = 0;
    while pos < word_bytes.len() {
        if pos + 1 < word_bytes.len() {
            pos += match word_bytes[pos] as char {
                'c' => c_start(&word_bytes, pos + 1),
                'd' => d_start(&word_bytes, pos + 1),
                'l' => l_start(&word_bytes, pos + 1),
                'n' => n_start(&word_bytes, pos + 1),
                's' => s_start(&word_bytes, pos + 1),
                'z' => z_start(&word_bytes, pos + 1),
                _ => 0,
            };
        }

        count += 1;
        pos += 1;
    }

    println!("{count}");
    Ok(())
}

fn c_start(word_bytes: &[u8], pos: usize) -> usize {
    match word_bytes[pos] as char {
        '=' | '-'  => 1,
        _ => 0,
    }
}

fn d_start(word_bytes: &[u8], pos: usize) -> usize {
    match word_bytes[pos] as char {
        '-' => 1,
        'z' => {
            if word_bytes.len() > pos + 1 && word_bytes[pos + 1] as char == '=' { return 2 } else { 0 }
        }
        _ => 0,
    }
}

fn l_start(word_bytes: &[u8], pos: usize) -> usize {
    match word_bytes[pos] as char {
        'j' => 1,
        _ => 0,
    }
}

fn n_start(word_bytes: &[u8], pos: usize) -> usize {
    match word_bytes[pos] as char {
        'j' => 1,
        _ => 0,
    }
}

fn s_start(word_bytes: &[u8], pos: usize) -> usize {
    match word_bytes[pos] as char {
        '=' => 1,
        _ => 0,
    }
}

fn z_start(word_bytes: &[u8], pos: usize) -> usize {
    match word_bytes[pos] as char {
        '=' => 1,
        _ => 0,
    }
}
