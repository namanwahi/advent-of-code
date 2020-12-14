use std::fs::File;
use std::str;
use std::io::{self, BufRead};
use std::error::Error;
use std::collections::HashMap;
use std::iter::successors;

fn check_time(timestamp: u64, bus_ids: &[u64]) -> bool {
    // PART 2 helper func
    for (i, bus_id) in bus_ids.iter().enumerate() {
        if (timestamp + i as u64) % bus_id != 0 {
            return false
        }
    }
    true
}

fn main() -> Result<(), Box<dyn Error>>{
    let f = File::open("inputs.txt")?;
    let mut lines = io::BufReader::new(f).lines().map(|line| line.unwrap());

    let start: u64 = lines.next().unwrap().parse()?;
    let bus_ids: Vec<Option<u64>> = lines.next().unwrap().split(",")
        .map(|s: &str| 
            match s {
                "x" => None,
                bus_id => Some(bus_id.parse::<u64>().unwrap()),
            }
        ).collect();

    // PART 1
    for timestamp in start..u64::MAX {
        let earliest_bus = bus_ids.iter().filter(|bus_id| if let Some(id) = bus_id { timestamp % id == 0 } else { false }).next();
        if let Some(Some(bus_id)) = earliest_bus {
            println!("Part 1: {}", bus_id * (timestamp - start));
            break;
        }
    }

    // PART 2
    let bus_ids: Vec<u64> = bus_ids.iter().map(|bus_id| if let Some(id) = bus_id { *id } else { 1 }).collect();
    let max_id: u64 = *bus_ids.iter().max().unwrap();
    let max_idx: usize = bus_ids.iter().position(|&val| val == max_id).unwrap();
    let earliest_departure_of_max: u64 = (start..u64::MAX).filter(|t| t % max_id == 0).next().unwrap();

    let part_2 = successors(Some(earliest_departure_of_max), |n| Some(n + max_id))
        .map(|departure_of_max| { println!("{}", departure_of_max); departure_of_max - max_idx as u64})
        .filter(|&first_departure| check_time(first_departure, &bus_ids))
        .next().unwrap();

    println!("{:?} {:?} {:?} {:?} {:?}", bus_ids, max_id, max_idx, earliest_departure_of_max, part_2);


    Ok(())
}
