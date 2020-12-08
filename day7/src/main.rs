#[macro_use]
extern crate nom;

use std::fs::File;
use std::str;
use std::io::{self, BufRead};
use std::error::Error;
use std::collections::HashMap;

use nom::{
    IResult,
    bytes::complete::tag,
    sequence::{tuple},
    character::{complete::{digit1, space0, space1, alpha1}},
};

// parsing macros
named!(colour_code<&[u8], (&[u8], char, &[u8])>, tuple!(alpha1, char!(' '), alpha1));
named!(parse_bag, alt!(tag!(" bags") | tag!(" bag")));
named!(parse_contents<&[u8], Vec<((&[u8], &[u8]), u32)>>, separated_list0!(tag(","), parse_one_content));

// parsing function
fn parse_colour_code(input: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
    let (input, (code_1, _, code_2)) = colour_code(input)?;
    Ok((input, (code_1, code_2)))
}

fn parse_one_content(input: &[u8]) -> IResult<&[u8], ((&[u8], &[u8]), u32)> {
    let (input, (_, count, _, code_1, _, code_2)) = tuple((space0, digit1, space1, alpha1, space1, alpha1))(input)?;
    let (input, _) = parse_bag(input)?;
    let count: u32 = str::from_utf8(count).unwrap().parse().unwrap();
    Ok((input, ((code_1, code_2), count)))
}

fn create_color_code(code_1: &[u8], code_2: &[u8]) -> String {
    format!("{} {}", str::from_utf8(code_1).unwrap(), str::from_utf8(code_2).unwrap())
}

fn parse_rule(input: &[u8]) -> IResult<&[u8], (String, HashMap<String, u32>)> {
    let (input, (code_1, code_2)) = parse_colour_code(input)?;
    let (input, _) = tuple((parse_bag, tag(" contain")))(input)?;
    let (input, contents) = parse_contents(input)?;
    let key = create_color_code(code_1, code_2);
    let vals = contents.into_iter().map(|((code_1, code_2), count)| (create_color_code(code_1, code_2), count)).collect();

    Ok((input, (key, vals)))
}


// program logic
fn contains_shiny_gold(bag: &String, bag_rules: &HashMap<String, HashMap<String, u32>>) -> bool {
    let contents = bag_rules.get(bag).unwrap();
    contents.contains_key(&String::from("shiny gold")) ||
        contents.iter().map(|(b, _)| contains_shiny_gold(b, bag_rules)).fold(false, |acc, x| x || acc)
}

fn nested_count(bag: &String, bag_rules: &HashMap<String, HashMap<String, u32>>) -> u32 {
    let contents = bag_rules.get(bag).unwrap();
    contents.iter().map(|(b, c)| c * nested_count(b, bag_rules)).sum::<u32>() + 1
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("inputs.txt")?;
    let reader = io::BufReader::new(f);

    let bag_rules: HashMap<_, _> = reader.lines()
            .map(|line| parse_rule(line.unwrap().as_bytes()).unwrap().1)
            .collect();

    let part1 = bag_rules.keys()
        .filter(|bag| contains_shiny_gold(bag, &bag_rules))
        .count();

    println!("{:?}", part1);

    let part2 = nested_count(&String::from("shiny gold"), &bag_rules) - 1;

    println!("{:?}", part2);

    Ok(())
}
