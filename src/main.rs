use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut input_map = HashMap::<String, String>::new();

    for line in reader.lines() {
        let line_result = line?;
        let mut iter = line_result.split("   ");

        while let (Some(key), Some(value)) = (iter.next(), iter.next()) {
            input_map.insert(key.to_string(), value.to_string());
        }
    }

    for (key, value) in input_map {
        println!("{}, {}", key, value);
    }

    Ok(())
}
