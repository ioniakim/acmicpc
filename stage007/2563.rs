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

        paste_color_paper(&mut white_paper, color_paper[0] as usize, color_paper[1] as usize);

    }

    let area: u16 = white_paper.iter().flatten().map(|&v| v as u16).sum();
    println!("{area}");
    Ok(())
}

fn paste_color_paper(white_paper: &mut [[u8; 100]; 100], x: usize, y: usize) {
    for i in x..x+10 {
        for j in y..y+10 {
            white_paper[j][i] = 1u8;
        }
    }
}
