use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let time: Vec<u16> = input.split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let hour: u16 = time[0];  // 0<= hour <= 23
    let minute: u16 = time[1];  // 0 <= minute <= 59

    input.clear();
    io::stdin().read_line(&mut input)?;

    let duration: u16 = input.trim().parse()?; // 0 <= duration <= 1000


    let duration_hour = duration / 60;
    let duration_minute = duration % 60;

    let mut end_minute = minute + duration_minute;
    let mut end_hour = (hour + duration_hour) % 24;

    if end_minute >= 60 {
        end_hour = if end_hour + 1 > 23 { 0 } else { end_hour + 1 };
        end_minute = end_minute - 60;
    }

    println!("{} {}", end_hour, end_minute);

    Ok(())

}
