use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let time: Vec<u16> = input.split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    if time.len() != 2 && !is_valid_time(time[0], time[1]) {
        return Err("Invalid time format".into());
    }

    let hour: u16 = time[0];  // 0<= hour <= 23
    let minute: u16 = time[1];  // 0 <= minute <= 59

    input.clear();
    io::stdin().read_line(&mut input)?;

    let duration: u16 = input.trim().parse()?; // 0 <= duration <= 1000

    let in_minutes = hour * 60 + minute + duration;

    let end_hour = (in_minutes / 60) % 24;
    let end_minute = in_minutes % 60;

    println!("{} {}", end_hour, end_minute);

    Ok(())
}

fn is_valid_time(hour: u16, minute: u16) -> bool {
    hour < 24 && minute < 60
}
