use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse()?;
    let mut numbers = vec![];
    for data in io::stdin().lines().take(n) {
        let data = data?;
        let number: i16 = data.parse()?;
        numbers.push(number);
    }

    sort_by_insertion(&mut numbers);
    numbers.iter().for_each(|&v| println!("{v}"));
    Ok(())
}

#[allow(dead_code)]
fn sort_by_insertion<T: std::cmp::PartialOrd>(numbers: &mut [T]) {
    let len = numbers.len();
    for i in 1..len {
        let mut i = i;
        while i > 0 && numbers[i-1] > numbers[i] {
            numbers.swap(i-1, i); // it works for arrays or slices
            // (numbers[i-1], numbers[i]) = (numbers[i], numbers[i-1]);
            i -= 1;
        }
    }
}

#[allow(dead_code)]
fn sort_by_bubble<T: std::cmp::PartialOrd>(numbers: &mut [T]) {
    let len = numbers.len();
    for i in 0..len {
        let mut min_index = i;
        for j in i+1..len {
            if numbers[min_index] > numbers[j] {
                min_index = j;
            }
        }
        numbers.swap(i, min_index);
    }
}
