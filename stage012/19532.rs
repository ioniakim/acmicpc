/// ax + by = c
/// dx + ey = f
/// x = (c - by) / a
/// d(c - by)/a + ey = f
/// dc/a - dby/a + ey = f
/// (e - db/a)y = f - dc/a
/// y = (f -dc/a) / (e - db/a)

use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let v: Vec<i32> = input.split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    // let (x, y) = match v[..] {
    //     [0, 0, 0, d, e, f] => find_by_combination(d, e, f),
    //     [a, b, c, 0, 0, 0] => find_by_combination(a, b, c),
    //     [0, b, c, d, e, f] => find_by_y(b, c, d, e, f),
    //     [a, b, c, 0, e, f] => find_by_y(e, f, a, b, c),
    //     [a, 0, c, d, e, f] => find_by_x(a, c, d, e, f),
    //     [a, b, c, d, 0, f] => find_by_x(d, f, a, b, c),
    //     [a, b, c, d, e, f] => find_by_formula(a, b, c, d, e, f),
    //     _ => return Err("Wrong Input".to_string().into()),
    // };

    let (x, y) = find_by_all_combination(v[0], v[1], v[2], v[3], v[4], v[5]);

    println!("{x} {y}");

    Ok(())
}

fn find_by_all_combination(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> (i32, i32) {
    for x in -999..1000 {
        for y in -999..1000 {
            if a * x + b * y == c && d * x + e * y == f {
                return (x, y)
            }
        }
    }
    (0, 0)
}

fn find_by_combination(p1: i32, p2: i32, p3: i32) -> (i32, i32) {
    for x in -999..1000 {
        for y in -999..1000 {
            if p1 * x + p2 * y == p3 {
                return (x, y)
            }
        }
    }
    (0, 0)
}

// 0 3 6 4 1 6 => y = 2 and x = (6 - 2) / 4 = 1
fn find_by_y(p1: i32, p2: i32, p3: i32, p4: i32, p5: i32) -> (i32, i32) {
    let y = p2 / p1;
    let x = (p5 - p4 * y) / p3;
    (x, y)
}

// 3 0 6 4 1 6 => x = 2 and y = 1
fn find_by_x(p1: i32, p2: i32, p3: i32, p4: i32, p5: i32) -> (i32, i32) {
    let x = p2 / p1;
    let y = (p5 - p4 * x) / p3;
    (x, y)
}

/// x = (c - by) / a
/// y = (f -dc/a) / (e - db/a)
fn find_by_formula(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> (i32, i32) {
    let y = (f - (d * c) / a) / (e - (d * b) / a);
    let x = (c - b * y) / a;
    (x, y)
}
