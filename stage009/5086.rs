use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    for line in io::stdin().lines() {
        let line = line?;

        let values: Vec<u16> = line.split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        if values[0] == 0 && values[1] == 0 {
            break;
        }

        let first = values[0];
        let second = values[1];

        if first < second && second % first == 0 {
            println!("factor");
        } else if first % second == 0 {
            println!("multiple");
        } else {
            println!("neither");
        }
    }
    Ok(())
}
