use std::fs::File;
use std::io::{self, BufRead, ErrorKind};
use std::error::Error;
use std::collections::{HashSet, HashMap};
use std::ops::Index;
use std::cmp;
use std::iter::{self,Chain};
use std::slice::Iter;
use std::fmt;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("inputs.txt")?;
    let reader = io::BufReader::new(f);

    /*
    let result = reader.lines()
        .map(|line| line.unwrap())
        .fold(vec![HashSet::new()], |mut acc, line| {
            match line.as_str() {
                "" => { acc.push(HashSet::new()); acc },
                _ => { acc.last_mut().unwrap().extend(line.chars()) ; acc }
            }
        })
        .iter()
        .fold(0, |acc, chars| acc + chars.len());
    */
    let result: usize = reader.lines()
        .map(|line| line.unwrap())
        .fold(vec![vec![]], |mut acc, line| {
            match line.as_str() {
                "" => { acc.push(vec![]); acc },
                _ => { acc.last_mut().unwrap().push(line.chars().collect::<HashSet<_>>()) ; acc }
            }
        })
        .iter()
        .map(|char_sets| {
            let all_chars: HashSet<char> = (b'a'..=b'z').map(|c| c as char).collect();
            char_sets.iter().fold(all_chars, |acc, char_set| acc.intersection(char_set).cloned().collect()).len()
        }).sum();

    println!("{:?}", result);   

    Ok(())
}