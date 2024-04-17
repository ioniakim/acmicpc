use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>()?;
    factorize_iterator_adapter(n);
    Ok(())
}

#[allow(dead_code)]
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

/// It doesn't work correctly. Only reference it as an example of an iterator usage
#[allow(dead_code)]
fn factorize_iterator(number: usize) {
    // Generate potential divisors up to the square root of the number
    let divisors = (2..=(number as f64).sqrt() as usize)
        .filter(|divisor| number % divisor == 0) // It can't allow repetitive factors. eg) if number is 4, 2 and 2 must be printed but only one 2.
        .inspect(|divisor| println!("{divisor}"));

    // Handle a potential remaing prime factor
    if let Some(last_factor) = divisors.last() {
        if last_factor * last_factor < number {
            println!("{}", number / last_factor);
        }
    }
}

/// It doesn't work correctly. Only reference it as an example of an iterator usage
///
#[allow(dead_code)]
fn factorize_iterator_adapter(number: usize) {
    (2..=(number as f64).sqrt() as usize)
        .take_while(|&i| i * i <= number)
        .filter(|&i| number % i == 0)
        .for_each(|i| {
            println!("{i}");
        });
}
