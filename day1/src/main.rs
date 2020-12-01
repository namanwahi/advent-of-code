use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("inputs.txt")?;
    let reader = io::BufReader::new(f);

    let mut seen = HashSet::new();

    for line in reader.lines() {
        if let Ok(num) = line {
            let num: u32 = num.parse()?;
            seen.insert(num);
            if seen.contains(&(2020 - num)) {
                println!("{}", num * (2020 - num));
                return Ok(())
            }
        }
    }
    Ok(())
}