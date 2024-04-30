fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut numbers = std::io::stdin().lines().take(5)
        .filter_map(Result::ok)
        .map(|line| line.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    sort_by_insertion(&mut numbers);

    println!("{}", numbers.iter().sum::<usize>() / numbers.len());
    println!("{}", numbers[2]);
    Ok(())
}

fn sort_by_insertion<T: std::cmp::PartialOrd>(elements: &mut [T]) {
    let len = elements.len();
    for i in 1..len {
        for j in (1..=i).rev() {
            if elements[j] >= elements[j - 1] {
                break;
            }
            elements.swap(j - 1, j);
        }
    }
}
