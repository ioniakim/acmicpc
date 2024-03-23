use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse()?;

    for line in io::stdin().lines().take(n) {
        let line = line?;
        let s = line.trim().to_owned();
        let first = s.chars().next().ok_or::<String>("first".into())?;
        let last = s.chars().last().ok_or::<String>("last".into())?;

        println!("{}{}", first, last);
    }

    Ok(())
}
