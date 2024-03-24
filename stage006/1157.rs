use std::io;

fn index(c: char) -> usize {
    c.to_ascii_lowercase() as usize - 'a' as usize
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let word = input.trim();

    const SIZE: usize = 'z' as usize - 'a' as usize + 1;
    let mut alpha_count: [u16; SIZE] = [0; SIZE];

    word.chars().for_each(|c| {
        alpha_count[index(c)] += 1;
    });

    let mut max_index = 0u8;
    let mut max_value = u16::MIN;
    let mut tie = false;
    for (idx, value) in alpha_count.iter().enumerate(){
        if max_value < *value {
            max_index = idx as u8;
            max_value = *value;
            tie = false;
        } else if max_value == *value {
            tie = true;
        }
    }

    if tie {
        println!("?");
    } else {
        println!("{}", ('A' as u8  + max_index) as char);
    }
    Ok(())
}
