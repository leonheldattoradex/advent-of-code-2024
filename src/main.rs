use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut input_map = HashMap::<String, String>::new();

    let mut counter = 0;

    for line in reader.lines() {
        let line_result = line?;
        let line_split = line_result.split("   ");

        let mut old_part: String = String::new();

        for part in line_split {
            counter += 1;
            if counter % 2 == 0 {
                input_map.insert(old_part.clone(), part.to_string());
            } else {
                old_part = part.to_string();
            }
        }

        println!("{}, {}", old_part, input_map[&old_part]);
    }

    Ok(())
}
