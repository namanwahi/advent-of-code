#[macro_use]
extern crate nom;

use std::fs::File;
use std::str;
use std::io::{self, BufRead};
use std::error::Error;
use std::collections::{HashSet, HashMap};

use nom::{
    IResult,
    bytes::complete::tag,
    sequence::{tuple},
    character::{complete::digit1},
};

// parser
named!(parse_instr<&[u8], (&[u8], char, char, &[u8]) >,
    tuple!(
        alt!( tag!("acc") | tag!("jmp") | tag!("nop") ),
        char!(' '),
        alt!( char!('+') | char!('-')),
        digit1
    )
);

// program logic
fn run_machine(instrs: &Vec<(String, i32)>) -> (i32, i32) {
    let mut ran_instrs = HashSet::<i32>::new();
    let (part1, terminated) = _run_machine(0, 0, instrs, &mut ran_instrs, None);

    assert!(!terminated, "Programme should have hit a loop");

    let part2 = ran_instrs.into_iter() // use the ran instructions set as candidates for the corrupt index instead of checking everyone one
        .filter(|ind| instrs[*ind as usize].0.as_str() != "acc") // filter jmp and nop instructions
        .map(|corrupt_ind| _run_machine(0, 0, instrs, &mut HashSet::new(), Some(corrupt_ind))) // run the machine with a corrupt index
        .filter(|(acc, terminated)| *terminated) // filter if terminated
        .collect::<Vec<(i32, bool)>>()[0].0; // get terminated accumulator

    (part1, part2)
}

fn _run_machine(acc: i32, instr_idx: i32, instrs: &Vec<(String, i32)>, ran_instrs: &mut HashSet<i32>, corrupt_idx: Option<i32>) -> (i32, bool) {
    // returns acc and a boolean if the programme terminated. Uses a set ran_instrs for cycle detection

    if !ran_instrs.insert(instr_idx) {
        return (acc, false)
    } else if (instr_idx as usize) == instrs.len() - 1 {
        return (acc, true)
    }

    let (op, arg) = &instrs[instr_idx as usize];
    let corrupt = corrupt_idx.is_some() && corrupt_idx.unwrap() == instr_idx;

    match (op.as_str(), corrupt) {
        ("acc", _) => _run_machine(acc + arg, instr_idx + 1, instrs, ran_instrs, corrupt_idx),
        ("jmp", false) | ("nop", true) => _run_machine(acc, instr_idx + arg, instrs, ran_instrs, corrupt_idx),
        ("nop", false) | ("jmp", true) => _run_machine(acc, instr_idx + 1, instrs, ran_instrs, corrupt_idx),
        _ => panic!("Halp"),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("inputs.txt")?;
    let reader = io::BufReader::new(f);

    let instructions: Vec<_> = reader.lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (_, (op, _, sign, u_arg)) = parse_instr(line.as_bytes()).unwrap();
            let op = String::from_utf8(op.to_vec()).unwrap();
            let arg = i32::from_str_radix(str::from_utf8(u_arg).unwrap(), 10).unwrap() * (if sign == '-' { -1 } else { 1 });
            (op, arg)
        }).collect();

    println!("{:?}", run_machine(&instructions));

    Ok(())
}
