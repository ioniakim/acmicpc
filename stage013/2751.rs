use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = std::io::BufWriter::new(std::io::stdout());
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input)?;
    // let n = input.trim().parse()?;

    // let mut data = std::io::stdin().lines().take(n)
    //     .filter_map(Result::ok)
    //     .map(|line| line.parse::<i32>())
    //     .collect::<Result<Vec<_>, _>>()?;

    let input = std::io::read_to_string(std::io::stdin())?;
    let mut numbers: Vec<i32> = input.trim().split_ascii_whitespace()
        .flat_map(str::parse)
        .collect();

    let data = &mut numbers[1..];

    // quick_sort(&mut data);
    data.sort();

    for e in data {
        writeln!(out, "{}", e)?;
    }
    Ok(())
}

#[allow(dead_code)]
fn quick_sort<T>(elements: &mut [T])
where T: std::cmp::PartialOrd {
    let start = 0;
    let end = elements.len();
    if start == end {
        return;
    }

    let p = partition(elements, start, end);

    quick_sort(&mut elements[start..p]);
    quick_sort(&mut elements[p + 1..end]);
}

#[allow(dead_code)]
fn partition<T>(elements: &mut [T], start: usize, end: usize) -> usize
where T: std::cmp::PartialOrd {
    let pivot = end - 1;

    let mut i = start;
    for j in start..end {
        if elements[j] < elements[pivot] {
            elements.swap(i, j);
            i += 1;
        }
    }

    elements.swap(i, pivot);
    i
}
