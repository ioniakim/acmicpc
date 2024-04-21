use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let play_data: Vec<usize> = input.split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    input.clear();
    io::stdin().read_line(&mut input)?;
    let cards: Vec<usize> = input.split_whitespace().take(play_data[0])
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let mut result = 0;
    'outer: for i in 0..cards.len()-2 {
        for j in i+1..cards.len()-1 {
            for k in j+1..cards.len() {
                let sum = cards[i] + cards[j] + cards[k];
                if sum == play_data[1] {
                    result = sum;
                    break 'outer;
                } else if sum > result && sum < play_data[1] {
                    result = sum;
                }
            }
        }
    }
    println!("{result}");
    Ok(())
}
