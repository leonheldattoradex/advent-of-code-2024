use regex::Regex;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = fs::read_to_string("./input.txt")?;

    input = input.replace("\n", "");

    // filter file to exclude muls between a don't() and a do()
    let re = Regex::new(r"don't\(\).*?do\(\)")?;

    let toggled_blocks_removed = re.replace_all(&input, "");

    println!("{}", toggled_blocks_removed);

    // match mul instructions
    let re = Regex::new(r"mul\(\d+,\d+\)")?;

    let results: Vec<_> = re
        .find_iter(&toggled_blocks_removed)
        .map(|mat| mat.as_str())
        .collect();

    // lazily match the arguments to the mul instruction again
    let re: Regex = Regex::new(r"\d+,\d+")?;

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
