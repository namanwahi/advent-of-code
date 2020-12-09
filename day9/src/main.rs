use std::fs::File;
use std::str;
use std::io::{self, BufRead};
use std::error::Error;
use std::collections::{HashSet};

fn main() -> Result<(), Box<dyn Error>>{
    let f = File::open("inputs.txt")?;
    let reader = io::BufReader::new(f);
    let nums: Vec<u64> = reader.lines().map(|line| line.unwrap().parse().unwrap()).collect();
    let window: usize = 25;

    let part1_idx = (window..nums.len())
        .find(|i: &usize| {
            let target = nums[*i];
            let nums = &nums[(*i - window)..*i];
            let (can_be_summed, _) = nums.into_iter()
                .fold((false, HashSet::new()), |(res, mut seen), num| {
                    let res = res || target >= *num && seen.contains(&(target - num));
                    let seen = {seen.insert(*num) ; seen};
                    (res, seen)
                });
            !can_be_summed
        })
        .unwrap();

    let invalid_num = nums[part1_idx];

    println!("part 1 {:?}", invalid_num);

    let cumsum: Vec<u64> = nums.iter().scan(0, |acc, &x| {*acc = *acc + x; Some(*acc)}).collect();
    let (low_cum, high_cum): (u64, u64) = cumsum.iter()
        .fold((None, HashSet::new()), |(res, mut seen), curr| {
            if res.is_some() {
                return (res, seen)
            }
            let search_for = if *curr >= invalid_num { curr - invalid_num } else { 0 };
            let res = if seen.contains(&search_for) { Some((search_for, *curr)) } else { None };
            let seen = { seen.insert(curr); seen };
            (res, seen)
        }).0.unwrap();

    let low_idx = cumsum.iter().position(|&x| x == low_cum).unwrap();
    let high_idx = cumsum.iter().position(|&x| x == high_cum).unwrap();
    let min = &nums[low_idx+1..high_idx+1].iter().min().unwrap();
    let max = &nums[low_idx+1..high_idx+1].iter().max().unwrap();

    println!("Part 2 {:?}", *min + *max);

    Ok(())
}
