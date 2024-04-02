use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut white_paper = [[0u8; 100]; 100];

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse()?;
    for line in io::stdin().lines().take(n) {
        let line = line?;
        let color_paper = line.split_whitespace()
            .map(str::parse::<u8>)
            .collect::<Result<Vec<_>, _>>()?;

        paste_color_paper(&mut white_paper, color_paper[0], color_paper[1]);

    }

    println!("{}", calculate_colored_area(&white_paper));
    Ok(())
}

fn paste_color_paper(white_paper: &mut [[u8; 100]; 100], x: u8, y: u8) {
    for i in x..x+10 {
        for j in y..y+10 {
            white_paper[j as usize][i as usize] = 1u8;
        }
    }
}

fn calculate_colored_area(white_paper: &[[u8; 100]; 100]) -> u16 {
    let mut area = 0u16;
    for i in 0..100 {
        for j in 0..100 {
            area += white_paper[j][i] as u16;
        }
    }
    area as u16
}
