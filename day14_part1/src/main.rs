#[macro_use]
extern crate nom;

use std::fs::File;
use std::str;
use std::io::{self, BufRead};
use std::error::Error;
use std::collections::{HashSet, HashMap};

use nom::{
    IResult,
    bytes::complete::{tag, take},
    sequence::{tuple},
    character::{complete::digit1},
};

fn parse_mask(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, _) = tag("mask = ")(input)?;
    let (input, mask) = take(36usize)(input)?;
    let on_mask = mask.chars().rev().enumerate().fold(0, |acc, (i, c)| if c == '1' { acc ^ (1 << i) } else { acc });
    let off_mask = !mask.chars().rev().enumerate().fold(0, |acc, (i, c)| if c == '0' { acc ^ (1 << i) } else { acc });
    Ok((input, (on_mask, off_mask)))
}

fn parse_mem(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, (_, loc, _, val)) = tuple((tag("mem["), digit1, tag("] = "), digit1))(input)?;
    Ok((input, (loc.parse().unwrap(), val.parse().unwrap())))
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("inputs.txt")?;
    let reader = io::BufReader::new(f);

    let mut mask: (u64, u64) = (0, !0);
    let mut mem: HashMap<u64, u64> = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if let Ok(("", new_mask)) = parse_mask(&line) {
            mask = new_mask;
            continue;
        } else if let Ok(("", (loc, val))) = parse_mem(&line) {
            let (on_mask, off_mask) = mask;
            let masked_val = (val | on_mask) & off_mask;
            mem.insert(loc, masked_val);
        }
    }

    println!("{:?}", mem.values().sum::<u64>());

    Ok(())
}

