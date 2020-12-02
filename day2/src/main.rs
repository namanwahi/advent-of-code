#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::{self, BufRead, ErrorKind};
use std::error::Error;
use std::collections::HashSet;
use std::ops::Index;
use std::cmp;
use std::iter::{self,Chain};
use std::slice::Iter;

use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n, take_until, take_till},
    combinator::map_res,
    sequence::{tuple, separated_pair},
    character::{complete::{digit1, space1, anychar}, is_alphanumeric},
    multi::{separated_list0,many1}
};

fn parse_condition(input: &str) -> IResult<&str, &str> {
    take_until(":")(input)
}

fn parse_range(input: &str) -> IResult<&str, (u8, u8)> {
    let (x, (low, high)) = separated_pair(digit1, tag("-"), digit1)(input)?;
    Ok((x, (low.parse().unwrap(), high.parse().unwrap())))
}

fn parse_line(input: &str) -> IResult<&str, (u8, u8, char, &str)> {
    let (input, condition): (&str, &str) = parse_condition(input)?;
    let (password, _) = take_till(|c: char| c.is_alphanumeric())(input)?;
    let (letter_to_count, range) = take_until(" ")(condition)?;
    let letter_to_count = letter_to_count.trim().chars().next().unwrap();
    let (_, (low, high)) = parse_range(range)?;
    Ok(("", (low, high, letter_to_count, password)))
}

fn check_line(low: u8, high: u8, c: char, password: &str) -> bool {
    // part 1
    //let matches = password.matches(c).collect::<Vec<_>>().len() as u8;
    //matches >= low && matches <= high

    let chars: Vec<_> = password.chars().collect();
    (chars[low as usize - 1] == c) ^ (chars[high as usize - 1] ==  c)
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("inputs.txt")?;
    let reader = io::BufReader::new(f);

    let count = reader.lines()
        .map(|l| {
            let l = l.unwrap().clone();
            let (_, parsed_line) = parse_line(&l).unwrap();
            check_line(parsed_line.0, parsed_line.1, parsed_line.2, parsed_line.3)
        })
        .filter(|b| *b == true)
        .collect::<Vec<_>>()
        .len();

    println!("count: {}", count);
    Ok(())
}