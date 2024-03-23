use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut alphabet_positions = vec![-1;
        'z' as usize - 'a' as usize + 1];

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input.trim().chars().enumerate()
        .map(|(i, c)| {
            let idx = c as usize - 'a' as usize;
            (i, idx)
        })
        .for_each(|(i, idx)| {
            if alphabet_positions[idx] == -1 {
                alphabet_positions[idx] = i as i8
            }
        });

    let result = alphabet_positions.iter()
        .map(i8::to_string)
        .collect::<Vec<_>>()
        .join(" ");

    println!("{result}");

    Ok(())
}
