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
    let one_locations = mask.chars().rev().enumerate().fold(0, |acc, (i, c)| if c == '1' { acc ^ (1 << i) } else { acc });
    let x_locations = mask.chars().rev().enumerate().fold(0, |acc, (i, c)| if c == 'X' { acc ^ (1 << i) } else { acc });
    Ok((input, (one_locations, x_locations)))
}

fn parse_mem(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, (_, loc, _, val)) = tuple((tag("mem["), digit1, tag("] = "), digit1))(input)?;
    Ok((input, (loc.parse().unwrap(), val.parse().unwrap())))
}

fn _add_floating_combinations(val: u64, bit_pos: u8, x_locations: u64, vals: &mut Vec<u64>) {
    // recursively add various combinations of the floating bit positions to a vector
    if bit_pos == 36 {
        vals.push(val);
        return
    } else if x_locations & (1 << bit_pos) == 0 {
        _add_floating_combinations(val, bit_pos + 1, x_locations, vals);
    } else {
        _add_floating_combinations(val | (1 << bit_pos), bit_pos + 1, x_locations, vals);
        _add_floating_combinations(val & !(1 << bit_pos), bit_pos + 1, x_locations, vals);
    }
}

fn get_all_values(val: u64, one_locations: u64, x_locations: u64) -> Vec<u64> {
    // get all the possible masked values based on the rules
    let mut res = vec![];
    _add_floating_combinations(val | one_locations, 0, x_locations, &mut res);
    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("inputs.txt")?;
    let reader = io::BufReader::new(f);

    let mut mask: (u64, u64) = (0, 0);
    let mut mem: HashMap<u64, u64> = HashMap::new();

    for line in reader.lines().map(|line| line.unwrap()) {
        if let Ok(("", new_mask)) = parse_mask(&line) {
            mask = new_mask;
        } else if let Ok(("", (loc, val))) = parse_mem(&line) {
            for loc in get_all_values(loc, mask.0, mask.1).into_iter() {
                mem.insert(loc, val);
            }
        }
    }

    println!("{:?}", mem.values().sum::<u64>());

    Ok(())
}

