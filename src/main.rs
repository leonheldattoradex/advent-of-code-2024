use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line_result = line?;
        let mut iter = line_result.split("   ");

        while let (Some(key), Some(value)) = (iter.next(), iter.next()) {
            a.push(key.to_string().parse::<i32>().unwrap());
            b.push(value.to_string().parse::<i32>().unwrap());
        }
    }

    a.sort();
    b.sort();

    let mut c: Vec<i32> = Vec::new();
    for (key, value) in a.iter().zip(b.iter()) {
        let r = key - value;
        c.push(r.abs());
    }

    let sum: i32 = c.iter().sum();
    println!("{}", sum);

    Ok(())
}
