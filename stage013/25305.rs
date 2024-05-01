///
///
///
///
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let mut iter = input.split_whitespace();
    let n = iter.next().expect("").parse::<usize>()?;
    let k = iter.next().expect("").parse::<usize>()? - 1;

    input.clear();
    std::io::stdin().read_line(&mut input)?;
    let mut numbers = input.split_whitespace()
        .map(str::parse::<u16>)
        .collect::<Result<Vec<_>, _>>()?;
    let kth_value = find_kth(&mut numbers, 0, n, k);
    println!("{kth_value}");
    Ok(())
}

fn find_kth<T>(elements: &mut [T], start: usize, end: usize, k: usize) -> T
where T: std::cmp::PartialOrd + Copy + std::fmt::Display + std::fmt::Debug
{
    let i = partition(elements, start, end);
    if k < i {
        return find_kth(elements, start, i, k);
    } else if k > i{
        return find_kth(elements, i+1, end, k);
    } else {
        return elements[k];
    }
}

fn partition<T>(elements: &mut [T], start: usize, end: usize) -> usize
where T: std::cmp::PartialOrd + Copy {
    let pivot_index = end - 1;
    let pivot = elements[pivot_index];
    let mut i = start;
    for j in start..end {
        if elements[j] > pivot {
            elements.swap(i, j);
            i += 1;
        }
    }
    elements.swap(i, pivot_index);
    i
}
