use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: u16 = input.trim().parse()?;

    println!("{} {}", count_packages(n), count_packages2(n));
    Ok(())
}

fn count_packages(n: u16) -> i16 {
    const FIVE: u16 = 5;
    const THREE: u16 = 3;

    let mut five_count = n / FIVE;
    if n % FIVE == 0 {
        return five_count as i16;
    }

    while five_count > 0 {
        let n = n - five_count * FIVE;
        if n % THREE == 0 {
            return (five_count + n / THREE) as i16;
        }
        five_count -= 1;
    }

    if n % THREE == 0 {
        return (n / THREE) as i16;
    }

    -1
}

fn count_packages2(n: u16) -> i16 {
    const FIVE: u16 = 5;
    const THREE: u16 = 3;
    let three_max = n / THREE;
    for three_count in 0..three_max {
        let remain = n - three_count * THREE;
        if remain % FIVE == 0 {
            return (remain / FIVE + three_count) as i16;
        }
    }
    if n % THREE == 0 {
        return three_max as i16;
    }

    -1
}
