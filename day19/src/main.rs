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
    sequence::{tuple, delimited},
    character::complete::{alpha1, digit1, anychar, space1},
    combinator::map_res,
    multi::{many1, separated_list1},
    branch::alt,
};

// PARSING AND STORAGE
#[derive(Debug)]
enum RuleBody {
    SingleChar(char),
    SubRules(Vec<Vec<usize>>),
}

fn single_char_rule(input: &str) -> IResult<&str, (usize, RuleBody)> {
    let (input, (rule_num, _, c, _)) = tuple((map_res(digit1, |s: &str| s.parse::<usize>()), tag(": \""), anychar, tag("\"")))(input)?;
    Ok((input, (rule_num, RuleBody::SingleChar(c))))
}

fn sub_rules(input: &str) -> IResult<&str, (usize, RuleBody)> {
    let (input, (rule_num, _, subrules)) = tuple((
        map_res(digit1, |s: &str| s.parse::<usize>()),
        tag(":"),
        separated_list1(
            tag(" |"),
            many1(
                map_res(
                    tuple((tag(" "), digit1)),
                    |(_, s) : (&str, &str)| s.trim().parse::<usize>()
                )
            ),
        )
    ))(input)?;

    Ok((input, (rule_num, RuleBody::SubRules(subrules))))
}

// PROGRAMME LOGIC

fn unfold_nested_rule(sub_rules: &Vec<Vec<usize>>, unfolds: usize, rule_num: usize) -> Vec<usize> {
    // expand a nested rule e.g. "11: 42 31 | 42 11 31" unfolded twice becomes "11: 42 42 42 31 31 31"
    if unfolds == 0 {
        return sub_rules[0].clone();
    }

    let nested_idx = sub_rules[1].iter().position(|&x| x == rule_num).unwrap();
    let mut res = vec![];
    sub_rules[1][..nested_idx].into_iter().for_each(|&x| { res.push(x); });
    unfold_nested_rule(sub_rules, unfolds - 1, rule_num).into_iter().for_each(|x| { res.push(x); });
    sub_rules[1][nested_idx+1..].into_iter().for_each(|&x| { res.push(x); });
    return res
}

fn matches<'a>(input: &'a str, rule_num: usize, rules: &'a HashMap<usize, RuleBody>) -> Vec<&'a str> {
    // matches a rule to an input. Returns a vector of the possible remaining strings after
    // the matched section
    let rule_body = &rules[&rule_num];
    match rule_body {
        RuleBody::SingleChar(c) => {
            if input.chars().next() == Some(*c) {
                return vec![&input[1..]]
            } else {
                return vec![]
            }
        },
        RuleBody::SubRules(sub_rules) => {
            let mut updated_sub_rules: &Vec<Vec<usize>> = &sub_rules;

            let mut unfolded_sub_rules: Vec<Vec<usize>> = vec![];
            if sub_rules.last().unwrap().contains(&rule_num) {
                // unfold nested rules if needed
                for unfolds in (0..input.len() + 1) {
                    unfolded_sub_rules.push(unfold_nested_rule(sub_rules, unfolds, rule_num));
                }
                updated_sub_rules = &unfolded_sub_rules;
            }

            return updated_sub_rules.iter().map(|sub_rule| {
                let mut rems = vec![input];
                for sub_rule_num in sub_rule {
                    let old_rems: Vec<_> = rems.drain(..).collect();
                    for old_rem in old_rems {
                        for sub_match in matches(old_rem, *sub_rule_num, rules) {
                            rems.push(sub_match);
                        }
                    }
                }
                return rems;
            }).flat_map(|s| s.into_iter()).collect();
        },
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("inputs.txt")?;
    let lines = io::BufReader::new(f).lines().map(|line| line.unwrap());

    let mut rules: HashMap<usize, RuleBody> = HashMap::new();
    let mut counter = 0;
    for line in lines {
        if line == "" {
            continue
        } else if let Ok(("", (rule_num, rule_body))) = alt((single_char_rule, sub_rules))(&line) {
            rules.insert(rule_num, rule_body);
        } else {
            if matches(&line, 0, &rules).contains(&"") {
                counter += 1;
            }
        }
    }

    println!("Counter {}", counter);

    Ok(())
}
