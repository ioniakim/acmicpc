use std::error::Error;
use std::io;
use std::cmp;

const BOARD_SIZE: usize = 50;

fn main() -> Result<(), Box<dyn Error>> {
    const SIZE: usize = 8;
    let mut board = [[false; BOARD_SIZE]; BOARD_SIZE];

    let (m, n) = build_board(&mut board)?;

    let mut min_count = usize::MAX;
    for i in 0..=m - SIZE {
        for j in 0..=n - SIZE {
            let count = count_change(&board, i, j, SIZE);
            min_count = cmp::min(min_count, count);
        }
    }
    println!("{min_count}");
    Ok(())
}

fn build_board(board: &mut [[bool; BOARD_SIZE]]) -> Result<(usize, usize), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let values = input.trim().split_whitespace()
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()?;

    for (i, line) in io::stdin().lines().take(values[0]).enumerate() {
        let line = line?;
        for (j, c) in line.chars().enumerate() {
            board[i][j] = if c == 'W' { true } else { false };
        }
    }

    Ok((values[0], values[1]))
}

fn count_change(board: &[[bool; BOARD_SIZE]], start_y: usize, start_x: usize, size: usize) -> usize {
    let mut count = 0usize;
    let base = board[start_y][start_x];
    for i in start_y..start_y + size {
        let base = if (i - start_y) % 2 == 0 { base } else { !base };
        for j in start_x..start_x + size {
            let base = if (j - start_x) % 2 == 0 { base } else { !base };
            if base != board[i][j] {
                count += 1;
            }
        }
    }
    cmp::min(count, size * size - count)
}

#[allow(dead_code)]
fn print_board(board: &[[bool; BOARD_SIZE]], m: usize, n: usize) {
    for i in 0..m {
        for j in 0..n {
            print!("{}", if board[i][j] { 'W' } else { 'B' });
        }
        println!();
    }
}
