use std::fs::File;
use std::str;
use std::io::{self, BufRead};
use std::error::Error;
use std::cmp::max;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>>{
    let f = File::open("inputs.txt")?;
    let reader = io::BufReader::new(f);
    let mut nums: Vec<i64> = reader.lines().map(|line| line.unwrap().parse().unwrap()).collect();
    nums.sort();
    nums.push(nums.last().unwrap() + 3);

    let (_, one_diffs, three_diffs) = nums.iter().fold((0, 0, 0), |(prev_num, one_diffs, three_diffs), &num| {
        match num - prev_num {
            1 => (num, one_diffs + 1, three_diffs),
            2 => (num, one_diffs, three_diffs),
            3 => (num, one_diffs, three_diffs + 1),
            _ => panic!("HALP"),
        }
    });

    println!("Part 1 {:?}", one_diffs * three_diffs);

    let part_2 = nums.iter()
        .fold(
            { let mut initial: HashMap<i64, u64> = HashMap::new() ; initial.insert(0, 1) ; initial },
            |mut acc, &num| {
                let path_count = (1..4).map(|i| acc.get(&(num - i)).unwrap_or(&0)).sum();
                acc.insert(num, path_count);
                acc
            }
        )[nums.last().unwrap()];

    println!("Part 2 {:?}", part_2);

    Ok(())
}
