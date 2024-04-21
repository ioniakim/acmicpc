use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>()?;

    let mut x_axis: Vec<isize> = vec![];
    let mut y_axis: Vec<isize> = vec![];

    for line in io::stdin().lines().take(n) {
        let line = line?;
        let mut axis = line.split_whitespace().flat_map(str::parse::<isize>);
        x_axis.push(axis.next().ok_or("Invalid".to_owned())?);
        y_axis.push(axis.next().ok_or("Invalid".to_owned())?);

    }

    if n == 1 {
        println!("0");
        return Ok(());
    }

    let width =  (x_axis.iter().max().ok_or("None".to_owned())? - x_axis.iter().min().ok_or("None".to_owned())?).abs();
    let height = (y_axis.iter().max().ok_or("None".to_owned())? - y_axis.iter().min().ok_or("None".to_owned())?).abs();

    println!("{}", width * height);
    Ok(())
}
