use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>()?;

    input.clear();
    std::io::stdin().read_line(&mut input)?;
    let origin = input.split_whitespace().take(n)
        .map(str::parse::<isize>)
        .collect::<Result<Vec<_>, _>>()?;

    let mut ordered = origin.clone();
    ordered.sort();

    ordered.dedup();

    let mut out = std::io::BufWriter::new(std::io::stdout());
    for e in origin {
        if let Some(i) = binary_search(&ordered, &e) {
            write!(out, "{} ", i)?
        }
    }
    writeln!(out)?;
    Ok(())
}


#[allow(dead_code)]
fn binary_search<T>(elements: &[T], e: &T) -> Option<usize>
where T: std::cmp::PartialOrd {
    if elements.is_empty() {
        return None;
    }
    let mid = elements.len() / 2;

    match &elements[mid] {
        v if v > e => binary_search(&elements[..mid], e),
        v if v < e => {
            match binary_search(&elements[mid + 1..], e) {
                Some(i) => Some(i + mid + 1),
                _ => None,
            }
        },
        _ => Some(mid),
    }
}
