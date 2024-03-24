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

    let max = alpha_count.iter().max().ok_or::<String>("Failed to find the max".into())?;
    let pos = alpha_count.iter().position(|c| c == max).ok_or::<String>("Failed to find the max".into())?;
    let rpos = alpha_count.iter().rposition(|c| c == max).ok_or::<String>("Failed to find the max reversely".into())?;
    if pos != rpos {
        println!("?");
    } else {
        println!("{}", ('A' as u8  + pos as u8) as char);
    }

    Ok(())
}
