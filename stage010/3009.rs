use std::io;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut xs = vec![];
    let mut ys = vec![];

    for line in io::stdin().lines().take(3) {
        let line = line?;
        let points = line.split_whitespace()
            .map(str::parse::<u16>)
            .collect::<Result<Vec<_>, _>>()?;
        xs.push(points[0]);
        ys.push(points[1]);
    }

    let x = find_point(xs);
    let y = find_point(ys);

    println!("{x} {y}");

    Ok(())
}

fn find_point(points: Vec<u16>) -> u16 {
    let mut point_count = HashMap::new();

    for &p in &points {
        point_count.entry(p).and_modify(|counter| *counter += 1).or_insert(1);
    }
    let the_p = points[0];
    if let Some((&p, _)) = point_count.iter().find(|(_, &count)| count == 1 ) { p } else { the_p }
}
