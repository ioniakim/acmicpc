use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = std::io::BufWriter::new(std::io::stdout());

    let input = std::io::read_to_string(std::io::stdin())?;
    let mut numbers: Vec<i32> = input.trim().split_ascii_whitespace()
        .flat_map(str::parse)
        .collect();

    let data = &mut numbers[1..];

    // data.sort();
    merge_sort(data);

    for e in data {
        writeln!(out, "{}", e)?;
    }
    Ok(())
}

static mut BUFFER: [i32; 1_000_000] = [0i32; 1_000_000];

fn merge_sort(elements: &mut [i32]) {
    if elements.len() == 1 {
        return;
    }
    let end = elements.len();
    let mid = elements.len() / 2;
    merge_sort(&mut elements[0..mid]);
    merge_sort(&mut elements[mid..end]);

    unsafe {
        let mut i = 0;
        let mut j = mid;
        let mut k = 0;
        while i < mid && j < end {
            if elements[i] <= elements[j] {
                BUFFER[k] = elements[i];
                i += 1;
            } else {
                BUFFER[k] = elements[j];
                j += 1;
            }
            k += 1;
        }
        if i < mid {
            BUFFER[k..k + mid - i].copy_from_slice(&elements[i..mid]);
        } else if j < end {
            BUFFER[k..k + end - j].copy_from_slice(&elements[j..end]);
        }
        elements[..end].copy_from_slice(&BUFFER[..end]);
    }
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
