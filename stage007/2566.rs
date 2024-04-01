use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut max = u8::MIN;
    let mut max_row = usize::MAX;
    let mut max_column = usize::MAX;

    for (i, line) in io::stdin().lines().take(9).enumerate() {
        let line = line?;
        for (j, value) in line.split_whitespace().take(9).map(str::parse::<u8>).enumerate() {
            let value = value?;
            if max <= value {
                max = value;
                max_row = i + 1;
                max_column = j + 1;
            }
        }
    }

    println!("{max}\n{max_row} {max_column}");

    Ok(())
}
