use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut line_levels: Vec<i32> = Vec::new();

    let mut safe_counter: i32 = 0;

    for line in reader.lines() {
        let line_result = line?;
        let mut iter = line_result.split(" ");

        while let Some(key) = iter.next() {
            line_levels.push(key.to_string().parse::<i32>().unwrap());
        }

        let mut prev: i32 = 0;

        let mut increasing: bool = true;
        let mut decreasing: bool = true;
        let mut safe_diff: bool = true;

        for level in &line_levels {
            let diff: i32;
            if prev != 0 {
                diff = level - prev;

                // monotomically increasing
                if diff > 0 {
                    increasing = true && increasing;
                } else {
                    increasing = false;
                }

                // monotomically decreasing
                if diff < 0 {
                    decreasing = true && decreasing;
                } else {
                    decreasing = false;
                }

                // adjacent levels are inside the safe range
                if diff.abs() >= 1 && diff.abs() <= 3 {
                    safe_diff = true && safe_diff;
                } else {
                    safe_diff = false;
                }
            }

            prev = *level;
        }

        if (increasing || decreasing) && safe_diff {
            safe_counter += 1;
        }

        line_levels.clear();
    }

    println!("{}", safe_counter);
    Ok(())
}
