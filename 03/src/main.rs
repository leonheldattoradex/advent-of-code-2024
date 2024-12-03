use regex::Regex;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("./input.txt")?;

    // match mul instructions
    let re = Regex::new(r"mul\(\d+,\d+\)")?;

    let results: Vec<_> = re.find_iter(&input).map(|mat| mat.as_str()).collect();

    // lazily match the arguments to the mul instruction again
    let re = Regex::new(r"\d+,\d+")?;

    let mut acc = 0;

    for result in results {
        let results: Vec<_> = re.find_iter(&result).map(|mat| mat.as_str()).collect();
        for new_results in results {
            let mut iter = new_results.split(",");
            while let (Some(value1), Some(value2)) = (iter.next(), iter.next()) {
                // println!("{}, {}", value1, value2)
                acc += (value1.to_string().parse::<i32>().unwrap()
                    * value2.to_string().parse::<i32>().unwrap())
            }
        }
    }

    println!("{}", acc);
    Ok(())
}
