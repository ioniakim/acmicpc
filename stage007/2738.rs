use std::io;
use io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::BufWriter::new(io::stdout());

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let matrix_size: Vec<usize> = input.split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let n: usize = matrix_size[0];
    let m: usize = matrix_size[1];

    // Create a two-dimensional vector in a statement
    // let mut mat1: Vec<Vec<u32>> = (0..n).map(|_| Vec::with_capacity(m)).collect();
    // let mut mat2: Vec<Vec<u32>> = (0..n).map(|_| Vec::with_capacity(m)).collect();

    let mut mat1: Vec<Vec<u32>> = Vec::with_capacity(n);
    let mut mat2: Vec<Vec<u32>> = Vec::with_capacity(n);

    for line in io::stdin().lines().take(n) {
        let line = line?;
        let row: Vec<u32> = line.split_whitespace()
            .take(m)
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        mat1.push(row);
    }

    for line in io::stdin().lines().take(n) {
        let line = line?;
        let row: Vec<u32> = line.split_whitespace()
            .take(m)
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        mat2.push(row);
    }

    let result = add_matrices(&mat1, &mat2);

    for row in result {
        let values = row.iter().map(u32::to_string).collect::<Vec<_>>().join(" ");
        writeln!(out, "{values}")?;
    }
    Ok(())
}

fn add_matrices(mat1: &Vec<Vec<u32>>, mat2: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    mat1.iter().zip(mat2.iter())
        .map(|(ref m1, ref m2)| {
            m1.iter().zip(m2.iter())
                .map(|(&v1, &v2)| v1 + v2)
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}
