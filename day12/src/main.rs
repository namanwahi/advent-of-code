#[macro_use] extern crate nom;
#[macro_use] extern crate maplit;

use std::fs::File;
use std::str;
use std::io::{self, BufRead};
use std::error::Error;
use std::collections::HashMap;

use nom::character::complete::digit1;

use ndarray::array;

// parser
named!(
    parse_instruction<&str, (char, i64)>,
    tuple!(
        alt!( char!('N') | char!('S') | char!('E') | char!('W') | char!('L') | char!('R') | char!('F') ),
        map_res!(digit1, |s: &str| s.parse::<i64>())
    )
);

fn main() -> Result<(), Box<dyn Error>>{
    let f = File::open("inputs.txt")?;
    let reader = io::BufReader::new(f);

    // ------------------ PARSE INSTRUCTIONS -----------------------------
    let instructions: Vec<(char, i64)> = reader.lines()
        .map(|line| parse_instruction(&line.unwrap()).unwrap().1 )
        .map(|instr|
            match instr {
                ('L', val) => ('R', 360 - val), // convert all Left rotations to right ones
                other => other,
            }
        ).collect();

    // ----------------- PART 1 ---------------------------------------
    let bearing_to_dir: HashMap<i64, _> = hashmap!{
        0 => array![0, 1],
        90 => array![1, 0],
        180 => array![0 , -1],
        270 => array![-1, 0],
    };

    let (_, pos) = instructions.iter()
        .fold(
            (90, array![0, 0]),
            |(bearing, pos), (action, val)| {
                match action {
                    'N' => (bearing, pos + array![0, *val]),
                    'S' => (bearing, pos + array![0, -*val]),
                    'E' => (bearing, pos + array![*val, 0]),
                    'W' => (bearing, pos + array![-*val, 0]),
                    'R' => ((bearing + *val) % 360, pos),
                    'F' => {
                        let dir = &bearing_to_dir[&bearing];
                        (bearing, pos + dir * *val)
                    }
                    _ => panic!("HALP"),
                }
            }
        );

    println!("Part 1: {}", pos[0].abs() + pos[1].abs());


    // ------------- PART 2 ----------------------------------------
    let (pos, _) = instructions.iter()
        .fold(
            (array![0, 0], array![10, 1]),
            |(pos, waypoint), (action, val)| {
                match action {
                    'N' => (pos, waypoint + array![0, *val]),
                    'S' => (pos, waypoint + array![0, -*val]),
                    'E' => (pos, waypoint + array![*val, 0]),
                    'W' => (pos, waypoint + array![-*val, 0]),
                    'F' => (pos + &waypoint * *val, waypoint),
                    'R' => {
                        // Apply rotation matrix as many times as desired
                        let num_rotations = *val / 90;
                        let rot_90 = array![[0, 1],
                                            [-1, 0]];
                        let waypoint = match num_rotations {
                            1 => rot_90.dot(&waypoint),
                            2 => rot_90.dot(&rot_90).dot(&waypoint),
                            3 => rot_90.dot(&rot_90).dot(&rot_90).dot(&waypoint),
                            _ => panic!("HALP"),
                        };
                        (pos, waypoint)
                    }
                    _ => panic!("HALP"),
                }
            }
        );

    println!("Part 2: {}", pos[0].abs() + pos[1].abs());

    Ok(())
}
