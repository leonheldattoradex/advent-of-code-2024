use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
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

    // clone and dedup the left list
    let mut a_clone = a.clone();
    a_clone.dedup();

    // iterate against the right list and count how many instances there are
    // for each deduped element in a
    let mut frequency_map = HashMap::<i32, i32>::new();
    let mut counter = 0;
    for key in a_clone.iter() {
        for value in b.iter() {
            if value == key {
                counter += 1;
            }
        }
        frequency_map.insert(*key, counter);
        println!("{}", counter);
        counter = 0;
    }

    let mut similarity_score: i32 = 0;
    // frequency_map is now a <element in left list, frequency in right list> map
    for (key, value) in a.iter().zip(b.iter()) {
        similarity_score += key * frequency_map.get(key).unwrap();
    }
    println!("{}", similarity_score);

    Ok(())
}
