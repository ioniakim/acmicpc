use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::BufWriter::new(io::stdout());
    let mut max_len = 0usize;
    const LINE: usize = 5;
    let mut board = [[0 as char; 15]; LINE];

    for (i, line) in io::stdin().lines().take(LINE).enumerate() {
        let line = line?;

        max_len = std::cmp::max(max_len, line.trim().len());
        line.trim().chars().enumerate()
            .for_each(|(j, c)| {
                board[i][j] = c;
            });
    }

    for j in 0..max_len {
        for i in 0..5 {
            if board[i][j] != 0 as char {
                write!(out, "{}", board[i][j])?;
            }
        }
    }
    writeln!(out, "")?;

    Ok(())
}
