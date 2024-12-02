use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let mut line_levels: Vec<i32> = Vec::new();

    let mut safe_global_counter: i32 = 0;

    for line in reader.lines() {
        let line_result = line?;
        let mut iter = line_result.split(" ");

        line_levels.clear();

        while let Some(key) = iter.next() {
            line_levels.push(key.to_string().parse::<i32>().unwrap());
        }

        let mut safe_report = false;

        for i in 0..line_levels.len() {
            let mut modified_levels = line_levels.clone();
            modified_levels.remove(i);

            if is_safe(&modified_levels) {
                safe_report = true;
                break;
            }
        }

        if safe_report {
            safe_global_counter += 1;
        }
    }

    println!("{}", safe_global_counter);
    Ok(())
}

fn is_safe(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return false;
    }

    let mut increasing = true;
    let mut decreasing = true;
    let mut safe_diff = true;

    for i in 1..levels.len() {
        let diff = levels[i] - levels[i - 1];

        if diff <= 0 {
            increasing = false;
        }

        if diff >= 0 {
            decreasing = false;
        }

        if diff.abs() < 1 || diff.abs() > 3 {
            safe_diff = false;
        }
    }

    (increasing || decreasing) && safe_diff
}
