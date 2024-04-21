use std::error::Error;
use std::io;
use std::cmp::{max, min};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>()?;

    let mut x_min = isize::MAX;
    let mut x_max = isize::MIN;
    let mut y_min = isize::MAX;
    let mut y_max = isize::MIN;

    for line in io::stdin().lines().take(n) {
        let line = line?;
        let mut axis = line.split_whitespace().flat_map(str::parse::<isize>);
        let x = axis.next().ok_or("Invalid".to_owned())?;
        x_min = min(x_min, x);
        x_max = max(x_max, x);

        let y = axis.next().ok_or("Invalid".to_owned())?;
        y_min = min(y_min, y);
        y_max = max(y_max, y);
    }

    if n == 1 {
        println!("0");
        return Ok(());
    }

    let width =  (x_max - x_min).abs();
    let height = (y_max - y_min).abs();

    println!("{}", width * height);
    Ok(())
}
