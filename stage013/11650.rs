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

    qsort_by(&mut points, two_tuple_less);
    // merge_sort_by(&mut points, two_tuple_less);

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

static mut BUFF: [(i32, i32); 100_001] = [(0, 0); 100_001];

#[allow(dead_code)]
fn merge_sort_by(elements: &mut[(i32, i32)], less: fn(&(i32, i32), &(i32, i32)) -> bool){
    let end = elements.len();
    if end < 10 {
        insertion_sort_by(elements, less);
        return;
    }

    let mid = end / 2;
    merge_sort_by(&mut elements[0..mid], less);
    merge_sort_by(&mut elements[mid..], less);

    let mut i = 0;
    let mut j = mid;
    let mut k = 0;
    unsafe {
        while i < mid && j < elements.len() {
            if less(&elements[i], &elements[j]) {
                BUFF[k] = elements[i];
                i += 1;
            } else {
                BUFF[k] = elements[j];
                j += 1;
            }
            k += 1;
        }

        if i < mid {
            BUFF[k..k + mid - i].copy_from_slice(&elements[i..mid]);
        } else if j < end {
            BUFF[k..k + end - j].copy_from_slice(&elements[j..]);
        }

        elements[0..].copy_from_slice(&BUFF[0..end]);
    }
}

#[allow(dead_code)]
fn gen_rand(seed: usize) -> impl FnMut() -> usize {
    const MULTIPLY: usize = 1664525;
    const INCREMENT: usize = 1013904223;
    let mut rand = seed;

    move || { rand = rand.wrapping_mul(MULTIPLY).wrapping_add(INCREMENT); rand }
}

#[allow(dead_code)]
fn shuffle<T>(elements: &mut[T], seed: usize) {
    let end = elements.len();
    let mut rand = gen_rand(seed);
    for i in 1..end {
        let p = rand() % (i + 1);
        elements.swap(p, i);
    }
}

#[allow(dead_code)]
fn qsort_by<T, F>(elements: &mut[T], less: F)
where F: Fn(&T, &T) -> bool {
    shuffle(elements, 1);

    quick_sort_by(elements, &less);
}

#[allow(dead_code)]
fn quick_sort_by<T, F>(elements: &mut [T], less: &F)
where F: Fn(&T, &T) -> bool {
    if elements.len() < 10 {
        insertion_sort_by(elements, less);
        return;
    }

    let p = partition(elements, less);

    quick_sort_by(&mut elements[0..p], less);
    quick_sort_by(&mut elements[p + 1..], less);
}

#[allow(dead_code)]
fn partition<T, F>(elements: &mut [T], less: F) -> usize
where F: Fn(&T, &T) -> bool {
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

fn insertion_sort_by<T, F>(elements: &mut [T], less: F)
where F: Fn(&T, &T) -> bool {
    for i in 1..elements.len() {
        for j in (1..=i).rev() {
            if less(&elements[j], &elements[j-1]) {
                elements.swap(j-1, j);
            }
        }
    }
}
