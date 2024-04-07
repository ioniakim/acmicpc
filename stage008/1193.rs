/// faction
/// 1/1, 1/2, 2/1, 3/1, 2/2, 1/3, 1/4, 2/3, 3/2, 4/1, ...
///
/// 1, 2, 3, 4, 5
/// even: numerator in ascending order
/// odd: denomiator in ascending order
///
/// 1, 1 + 2, 3 + 3, 6 + 4, 10 + 5
///
///
/// seed = 1..
/// numerator = 1..=seed
/// denominator = (1..=seed).rev()
/// count += 1

use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let x = input.trim().parse::<u32>()?;

    // Locate the turn and the max value
    let mut turn_max = 1;
    let mut turn = 1;
    while turn_max < x {
        turn += 1;
        turn_max += turn;
    }

    // Find numerator and denomiator according to the turn.
    match turn % 2 {
        0 => {
            let denominator = turn_max - x + 1;
            let numerator = turn - denominator + 1;
            println!("{numerator}/{denominator}");
        },
        _ => {
            let numerator = turn_max - x + 1;
            let denominator = turn - numerator + 1;
            println!("{numerator}/{denominator}");
        }
    }
    Ok(())
}


