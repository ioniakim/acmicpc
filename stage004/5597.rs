use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut project = vec![false; 30];

    io::stdin().lines()
        .take(28)
        .flatten()  // Rip off the Result and leave the value only.
        .for_each(|s| {
            let idx: usize = s.trim().parse().expect("Failed to parse");
            project[idx - 1] = true;
        });

    project.into_iter()
        .enumerate()
        .filter(|(_, submit)| !submit)
        .for_each(|(idx, _)| println!("{}", idx + 1));

    Ok(())
}
