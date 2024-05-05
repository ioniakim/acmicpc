/// [ ] &str and &String are not compatible in function pointer
/// [ ] ways to return a closure. eg) using impl, Box<dyn>. why not specifying in where clause
use std::cmp::Ordering;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>()?;

    let mut strings: Vec<String> = std::io::stdin().lines().take(n)
        .flat_map(|s| s)
        .collect();

    strings.sort_by(length_lexical_order);

    strings.dedup();

    let mut out = std::io::BufWriter::new(std::io::stdout());
    for s in strings {
        writeln!(out, "{s}")?;
    }
    Ok(())
}

/// Have to find to use &str instead of &&str
fn length_lexical_order(s1: &String, s2: &String) -> std::cmp::Ordering {
    // length order
    if s1.len() < s2.len() {
        return Ordering::Less;
    } else if s1.len() > s2.len() {
        return Ordering::Greater;
    }

    // lexical order
    let mut two_chars = s1.bytes().zip(s2.bytes());

    while let Some((c1, c2)) = two_chars.next() {
        if c1 < c2 {
            return Ordering::Less;
        } else if c1 > c2 {
            return Ordering::Greater;
        }
    }

    Ordering::Equal
}

#[allow(dead_code)]
fn shuffle_qsort_by<T, F>(elements: &mut [T], cmp: F)
where F: Fn(&T, &T) -> std::cmp::Ordering {
    shuffle(elements);

    qsort_by(elements, &cmp);
}

#[allow(dead_code)]
fn gen_rand(seed: usize) -> impl FnMut() -> usize {
    const MULTIPLY: usize = 1664525;
    const INCREMENT: usize = 1013904223;
    let mut rand = seed;
    return move || {
        rand = rand.wrapping_mul(MULTIPLY).wrapping_add(INCREMENT);
        rand
    }
}

#[allow(dead_code)]
fn shuffle<T>(elements: &mut [T]) {
    let end = elements.len();
    let mut rand = gen_rand(1024);
    for i in 1..end {
        for j in (1..i).rev() {
            let p = rand() % (j + 1);
            elements.swap(j, p);
        }
    }
}

#[allow(dead_code)]
fn qsort_by<T, F>(elements: &mut [T], cmp: &F)
where F: Fn(&T, &T) -> std::cmp::Ordering {
    if elements.len() < 10 {
        insertion_sort_by(elements, cmp);
        return;
    }

    let p = partition_by(elements, cmp);

    qsort_by(&mut elements[..p], cmp);
    qsort_by(&mut elements[p+1..], cmp);
}

#[allow(dead_code)]
fn partition_by<T, F>(elements: &mut [T], cmp: F) -> usize
where F: Fn(&T, &T) -> std::cmp::Ordering {
    let pivot = 0;
    let end = elements.len();
    let mut i = 1;
    let mut j = end - 1;
    loop {
        while cmp(&elements[i], &elements[pivot]) == Ordering::Less {
            if i == end - 1 { break; }
            i += 1;
        }
        while cmp(&elements[pivot], &elements[j]) == Ordering::Less {
            j -= 1;
        }

        if i >= j { break; }
        elements.swap(i, j);
    }
    elements.swap(j, pivot);
    j
}

#[allow(dead_code)]
fn insertion_sort_by<T, F>(elements: &mut [T], cmp: F)
where F: Fn(&T, &T) -> std::cmp::Ordering {
    let end = elements.len();
    for i in 1..end {
        for j in (1..=i).rev() {
            if cmp(&elements[j], &elements[j-1]) != Ordering::Less {
                break;
            }
            elements.swap(j, j-1);
        }
    }
}
