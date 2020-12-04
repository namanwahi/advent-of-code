#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::{self, BufRead, ErrorKind};
use std::error::Error;
use std::collections::{HashSet, HashMap};
use std::ops::Index;
use std::cmp;
use std::iter::{self,Chain};
use std::slice::Iter;
use std::fmt;

use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n, take_until, take_till, take},
    combinator::map_res,
    sequence::{tuple, separated_pair},
    character::{complete::{digit1, space1, anychar, one_of}, is_alphanumeric},
    multi::{separated_list0,many1,many_m_n},
    branch::{alt, permutation},
};

#[derive(Debug)]
struct ParsingError;

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Parsing error")
    }
}

impl Error for ParsingError {
    fn description(&self) -> &str {
        "Parsing Error"
    }
}

fn parse_bounded_int(input: &str, low: u32, high: u32) -> Result<u32, Box<dyn Error>> {
    let year = u32::from_str_radix(input, 10)?;
    if year < low || year > high {
        return Err(Box::new(ParsingError))
    }
    Ok(year)
}

// parsers
fn iyr(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag(" iyr:")(input)?;
    map_res(digit1, |i| parse_bounded_int(i, 2010, 2020))(input)
}

fn byr(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag(" byr:")(input)?;
    map_res(digit1, |i| parse_bounded_int(i, 1920, 2002))(input)
}

fn eyr(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag(" eyr:")(input)?;
    map_res(digit1, |i| parse_bounded_int(i, 2020, 2030))(input)
}

fn ecl(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag(" ecl:")(input)?;
    alt((tag("amb"), tag("blu"), tag("brn"), tag("gry"), tag("grn"), tag("hzl"), tag("oth")))(input)
}

fn pid(input: &str) -> IResult<&str, Vec<char>> {
    let (input, _) = tag(" pid:")(input)?;
    many_m_n(9, 9, one_of("0123456789"))(input)
}

fn hcl(input: &str) -> IResult<&str, Vec<char>> {
    let (input, _) = tag(" hcl:#")(input)?;
    many_m_n(6, 6, one_of("0123456789abcdef"))(input)
}

fn cid(input: &str) -> IResult<&str, Vec<char>> {
    if input.is_empty() {
        return Ok(("", vec![]))
    }

    let (input, _) = tag(" cid:")(input)?;
    many_m_n(1, 3, one_of("0123456789"))(input)
}

fn hgt(input: &str) -> IResult<&str, (u32, &str)> {
    let (input, _) = tag(" hgt:")(input)?;
    alt((
        tuple((map_res(digit1, |i| parse_bounded_int(i, 150, 193)), tag("cm"))),
        tuple((map_res(digit1, |i| parse_bounded_int(i, 59, 76)), tag("in")))
    ))(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("inputs.txt")?;
    let reader = io::BufReader::new(f);

    let passports = reader.lines()
        .map(|line| line.unwrap())
        .fold(vec![String::new()], |mut acc, line| {
            match line.as_str() {
                "" => { acc.push(String::new()); acc },
                _ => { acc.last_mut().unwrap().push_str(format!(" {}", &line).as_str()) ; acc }
            }
        })
        .iter()
        .filter(|passport| permutation((pid, hgt, ecl, eyr, byr, hcl, iyr, cid))(passport).is_ok())
        .count();

    println!("{:?}", passports);
    Ok(())
}
