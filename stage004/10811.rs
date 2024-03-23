use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let first_line: Vec<u8> = input.split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let n = first_line[0];
    let m = first_line[1] as usize;

    let mut baskets: Vec<u8> = (0..=n).collect();

    for line in io::stdin().lines().take(m) {
        let line = line?;

        let terms: Vec<usize> = line.split_whitespace()
            .flat_map(str::parse)
            .collect::<Vec<_>>();

        baskets[terms[0]..=terms[1]].reverse();
    }

    println!("{}", baskets[1..].iter().map(u8::to_string).collect::<Vec<_>>().join(" "));
    Ok(())
}
