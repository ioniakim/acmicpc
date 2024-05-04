use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>()?;

    let mut points: Vec<(i32, i32)> = vec![];
    for line in std::io::stdin().lines().take(n) {
        let line = line?;
        let mut iter = line.split_whitespace();
        points.push((iter.next().unwrap().parse()?, iter.next().unwrap().parse()?));
    }

    quick_sort_by(&mut points, two_tuple_less);

    let mut out = std::io::BufWriter::new(std::io::stdout());
    for (x, y) in points {
        writeln!(out, "{x} {y}")?;
    }
    Ok(())
}

fn two_tuple_less<T>(t1: &(T, T), t2: &(T, T)) -> bool
where T: std::cmp::PartialOrd {
    if (t1.0 < t2.0) || (t1.0 == t2.0 && t1.1 < t2.1) {
        true
    } else {
        false
    }
}

#[allow(dead_code)]
fn quick_sort_by<T>(elements: &mut [T], less: fn(&T, &T) -> bool) {
    if elements.len() < 10 {
        insertion_sort_by(elements, less);
        return;
    }

    let p = partition(elements, less);

    quick_sort_by(&mut elements[0..p], less);
    quick_sort_by(&mut elements[p + 1..], less);
}

#[allow(dead_code)]
fn partition<T>(elements: &mut [T], less: fn(&T, &T) -> bool) -> usize{
    let lo = 0;
    let hi = elements.len() - 1;
    let pivot = lo;
    let mut i = lo + 1;
    let mut j = hi;

    loop {
        while less(&elements[i], &elements[pivot]) {
            if i == hi { break; }
            i += 1;
        }
        while less(&elements[pivot], &elements[j]) {
            j -= 1;
        }
        if i >= j { break; }
        elements.swap(i, j);
    }
    elements.swap(j, pivot);
    j
}

fn insertion_sort_by<T>(elements: &mut [T], less: fn(&T, &T) -> bool) {
    for i in 1..elements.len() {
        for j in (1..=i).rev() {
            if less(&elements[j], &elements[j-1]) {
                elements.swap(j-1, j);
            }
        }
    }
}
