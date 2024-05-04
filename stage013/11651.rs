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

    shuffle_qsort_by(&mut points, point_y_less);

    let mut out = std::io::BufWriter::new(std::io::stdout());
    for (x, y) in points {
        writeln!(out, "{x} {y}")?;
    }

    Ok(())
}

fn gen_rand(seed: usize) -> Box<dyn FnMut() -> usize> {
    const MULTIPLY: usize = 1664525;
    const INCREMENT: usize = 1013904223;
    let mut rand = seed;

    Box::new(move || { rand = rand.wrapping_mul(MULTIPLY).wrapping_add(INCREMENT); rand })
}

fn shuffle<T>(elements: &mut[T], seed: usize) {
    let end = elements.len();
    let mut rand = gen_rand(seed);
    for i in 1..end {
        let p = rand() % (i + 1);
        elements.swap(i, p);
    }
}

fn point_y_less(p1: &(i32, i32), p2: &(i32, i32)) -> bool {
    if p1.1 < p2.1 || (p1.1 == p2.1 && p1.0 < p2.0) {
        true
    } else {
        false
    }
}

fn shuffle_qsort_by<T>(elements: &mut [T], less: fn(&T, &T) -> bool) {
    shuffle(elements, 1024);

    qsort_by(elements, less);
}

fn qsort_by<T>(elements: &mut [T], less: fn(&T, &T) -> bool) {
    if elements.len() < 10 {
        insertion_sort(elements, less);
        return;
    }

    let p = partition_by(elements, less);

    qsort_by(&mut elements[..p], less);
    qsort_by(&mut elements[p+1..], less);
}

fn partition_by<T>(elements: &mut [T], less: fn(&T, &T) -> bool) -> usize {
    let end = elements.len();
    let pivot = 0;
    let mut i = 1;
    let mut j = end - 1;

    loop {
        while less(&elements[i], &elements[pivot]) {
            if i == end - 1 { break; }
            i += 1;
        }
        while less(&elements[pivot], &elements[j]) {
            j -= 1;
        }

        if i >= j {
            break;
        }
        elements.swap(i, j);
    }
    elements.swap(j, pivot);
    j
}


fn insertion_sort<T>(elements: &mut [T], less: fn(&T, &T)->bool) {
    let end = elements.len();
    for i in 1..end {
        for j in (1..=i).rev() {
            if less(&elements[j], &elements[j - 1]){
                elements.swap(j, j - 1);
            }
        }
    }
}
