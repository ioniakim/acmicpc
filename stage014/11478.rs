use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input)?;
    let input = input.trim();
    let mut substr = HashSet::new();
    for i in 0..input.len(){
        for j in i + 1..=input.len() {
            substr.insert(&input[i..j]);
        }
    }

    println!("{}", substr.len());

    Ok(())
}
