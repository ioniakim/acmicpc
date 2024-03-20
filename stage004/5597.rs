use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut project = vec![false; 30];

    let mut input = String::new();
    for _ in 0..28 {
        io::stdin().read_line(&mut input)?;
        let submit: usize = input.trim().parse()?;
        input.clear();
        project[submit - 1] = true;
    }

    project.into_iter()
        .enumerate()
        .filter(|(_, submit)| !submit)
        .for_each(|(idx, _)| println!("{}", idx + 1));

    Ok(())
}
