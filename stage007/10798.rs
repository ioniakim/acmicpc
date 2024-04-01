use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    (0..max_len).for_each(|j| {
        (0..5).map(|i| board[i][j])
            .filter(|&c| c != 0 as char)
            .for_each(|c| print!("{c}"))
    });

    Ok(())
}
