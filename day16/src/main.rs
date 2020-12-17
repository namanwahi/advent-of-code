use std::io::{self, BufRead};
use std::error::Error;
use std::fs::File;
use std::collections::{HashMap, HashSet};

extern crate nom;
use nom::{
  IResult,
  bytes::complete::{tag, take_until},
  character::complete::{digit1, char},
  combinator::map_res,
  sequence::tuple,
  multi::separated_list1,
};

use itertools::Itertools;

fn parse_rule(input: &str) -> IResult<&str, (&str, u64, u64, u64, u64)> {
    let (_, (field, _, low_1, _, high_1, _, low_2, _, high_2)) = tuple((
        take_until(":"),
        tag(": "),
        map_res(digit1, |s: &str| s.parse::<u64>()),
        char('-'),
        map_res(digit1, |s: &str| s.parse::<u64>()),
        tag(" or "),
        map_res(digit1, |s: &str| s.parse::<u64>()),
        char('-'),
        map_res(digit1, |s: &str| s.parse::<u64>()),
    ))(input)?;

    Ok((input, (field, low_1, high_1, low_2, high_2)))
}

fn parse_tickets(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(","), map_res(digit1, |s: &str| s.parse::<u64>()))(input)
}

fn main() -> Result<(), Box<dyn Error>>{
    let f = File::open("inputs.txt")?;
    let lines = io::BufReader::new(f).lines().map(|line| line.unwrap());

    // parse data
    let mut rules: HashMap<String, ((u64, u64), (u64, u64))> = HashMap::new();
    let mut tickets: Vec<Vec<u64>> = Vec::new();
    for line in lines {
        if let Ok((_, (field, low_1, high_1, low_2, high_2))) = parse_rule(&line) {
            rules.insert(String::from(field), ((low_1, high_1), (low_2, high_2)));
        } else if let Ok((_, vals)) = parse_tickets(&line) {
            tickets.push(vals);
        }
    }

    // part 1
    let part_1: u64 = tickets.iter()
        .skip(1) // skip your ticket
        .flat_map(|ticket| ticket.iter())
        .filter(|&num| {
            let in_any_range = rules.values().fold(false, |res, ((l1, h1), (l2, h2))| {
                res || (l1 <= num && num <= h1) || (l2 <= num && num <= h2)
            });
            !in_any_range
        })
        .sum();

    println!("Part 1 {:?}", part_1);

    // part 2

    // filter valid nearby tickets
    let valid_nearby_tickets: Vec<_> = tickets.iter().skip(1).filter(|&ticket| {
        for num in ticket.iter() {
            let in_any_range = rules.values().fold(false, |res, ((l1, h1), (l2, h2))| {
                res || (l1 <= num && num <= h1) || (l2 <= num && num <= h2)
            });
            if !in_any_range {
                return false;
            }
        }
        true
    }).collect();

    // create a vector of all possible features at each index
    let mut idx_to_fields = vec![];
    let num_fields = rules.len();
    for i in (0..num_fields) {
        let fields: HashSet<_> = rules.keys()
            .filter(|field| {
                for ticket in valid_nearby_tickets.iter() {
                    let ((l1, h1), (l2, h2)) = *rules.get(*field).unwrap();
                    let num = ticket[i];
                    if !((l1 <= num && num <= h1) || (l2 <= num && num <= h2)) {
                        return false
                    }
                }
                true
            }).collect();
        idx_to_fields.push(fields);
    }

    // create a mapping of fields to the only field index they can correspond to
    let mut fields_assigned: HashMap<_, usize> = HashMap::new();
    for elems_in_set in (1..num_fields + 1) {

        // find the set with n elements
        let (idx, curr_fields) = idx_to_fields.iter().enumerate().find(|&r| r.1.len() == elems_in_set).unwrap();

        // if n > 1, find the set with n - 1 elements and use the set difference to find the current field to assign
        let curr_field = if elems_in_set == 1 { 
            curr_fields.iter().next().unwrap() 
        } else {
            let prev_fields = idx_to_fields.iter().find(|&r| r.len() == elems_in_set - 1).unwrap();
            curr_fields.difference(prev_fields).next().unwrap()
        };

        fields_assigned.insert(curr_field, idx);
    }

    let part_2: u64 =  fields_assigned.keys()
        .filter(|field| field.starts_with("departure"))
        .map(|field| tickets[0][fields_assigned[field]])
        .product();

    println!("Part 2 {:?}", part_2);

    Ok(())
}
