use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>()?;
    factorize_while(n);
    Ok(())
}

fn factorize_while(number: usize) {
    let mut number = number;
    let mut divisor = 2;
    while divisor * divisor <= number {
        while number % divisor == 0 {
            number /= divisor;
            println!("{divisor}");
        }

        divisor += 1;
    }
    if number != 1 {
        println!("{number}");
    }
}
