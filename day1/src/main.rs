use std::fs::File;
use std::io::{self, BufRead, ErrorKind};
use std::error::Error;
use std::collections::HashSet;
use std::ops::Index;
use std::cmp;
use std::iter::{self,Chain};
use std::slice::Iter;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("inputs.txt")?;
    let reader = io::BufReader::new(f);

    let nums: Vec<i32> = reader.lines()
        .map(|line| {
            let line = line.unwrap();
            let num: i32 = line.parse().unwrap();
            num
        })
        .collect();

    println!("Part 1");
    if let Some(p1) = part1(2020, &nums, &[]) {
        println!("Answer: {}\n", p1)
    };
    println!("Part 2");
    if let Some(p2) = part2(2020, &nums) {
        println!("Answer: {}\n", p2)
    };


    Ok(())
}

fn part1(target: i32, nums: &[i32], more_nums: &[i32]) -> Option<i32> {
    let mut seen = HashSet::new();
    for num in nums.iter().chain(more_nums.iter()) {
        if seen.contains(&(target - num)) {
            return Some(num * (target - num))
        }
        seen.insert(num);
    }
    None
}

fn part2(target: i32, nums: &[i32]) -> Option<i32> {
    for (i, e) in nums.iter().enumerate() {
        if let Some(p1) = part1(target - e, &nums[..1], &nums[i+1..]) {
            return Some(p1 * e)
        }
    }
    None
}