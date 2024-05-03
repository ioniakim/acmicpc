fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let mut value = input.trim().parse::<usize>()?;

    let mut count = [0u8; 10];
    while value > 0 {
        count[value % 10] += 1;
        value /= 10;
    }

    for (i, &c) in count.iter().enumerate().rev() {
        for _ in 0..c {
            print!("{i}");
        }
    }
    println!();
    Ok(())
}
