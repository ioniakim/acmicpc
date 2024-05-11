use std::io::{
    stdin,
    stdout,
    read_to_string,
    BufWriter,
    Write,
    BufReader,
};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(BufReader::new(stdin()))?;
    let mut iter = input.split_ascii_whitespace();
    let mut next = || iter.next().ok_or::<String>("No More Input".into());

    let n: usize = next()?.parse()?;
    let m: usize = next()?.parse()?;

    let poketmons = (0..n).map(|_| next()).collect::<Result<Vec<_>, _>>()?;
    let name_map = poketmons.iter().enumerate()
        .map(|(no, name)| (name, no + 1))
        .collect::<HashMap<&&str, usize>>();

    let mut out = BufWriter::new(stdout());
    for _ in 0..m {
        let input = next()?;
        match input.parse::<usize>() {
            Ok(no) => writeln!(out, "{}", poketmons[no - 1])?,
            _ => writeln!(out, "{}", name_map.get(&input).ok_or::<String>("No matching name".into())?)?,
        }
    }

    Ok(())
}


// todo study the below code to improve performance

// use std::{io, io::prelude::*};
// use std::collections::HashMap;

// fn main() {
//     let mut stdin
// 	= io::stdin().lock();
//     let mut stdout
// 	= io::stdout().lock();

//     let mut buffer = String::with_capacity(1024 * 1024 * 40);
//     let mut output_buffer = Vec::new();

//     stdin.read_to_string(&mut buffer).expect("read failed");
//     let mut lines
// 	= buffer.lines().map(str::trim);

//     let mut first
// 	= lines.next().unwrap().split_whitespace();

//     let n : usize = first.next().unwrap().parse().unwrap();
//     let m : usize = first.next().unwrap().parse().unwrap();


//     let mut vec = Vec::new();
//     let mut map = HashMap::new();

//     for (i,line) in (&mut lines).take(n).enumerate() {
// 	vec.push(line);
// 	map.insert(line, i + 1);
//     }

//     lines.take(m)
// 	.for_each(|line| {
// 	    let _ = match line.parse::<usize>() {
// 		Ok(num) => writeln!(output_buffer, "{}", vec[num - 1]),
// 		Err(_) => writeln!(output_buffer, "{}", map[line]),
// 	    };
// 	});

//     let _ = io::copy(&mut output_buffer.as_slice(), &mut stdout);
// }
