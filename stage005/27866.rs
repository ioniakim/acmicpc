use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let word = input.trim().to_string();

    input.clear();
    io::stdin().read_line(&mut input)?;

    let mut ith: usize = input.trim().parse()?;
    ith -= 1;

    let c = word.chars().nth(ith).ok_or::<String>("Failed".into())?;
    println!("{c}");

    // if let Some(c) = word.chars().nth(ith) {
    //     println!("{c}");
    // }

    Ok(())
}
