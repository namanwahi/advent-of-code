use std::fs::File;
use std::str;
use std::io::{self, BufRead};
use std::error::Error;
use std::collections::HashMap;

use num::integer::lcm;

fn check_time(timestamp: u64, bus_ids: &[u64]) -> bool {
    // PART 2 helper func - returns true if bus_ids run continuously past timestamp
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

    // replace 'x's with bus_ids of 1
    let bus_ids: Vec<u64> = bus_ids.iter().map(|bus_id| if let Some(id) = bus_id { *id } else { 1 }).collect();
    let mut tstamp = start;
    let mut step = 1;
    for (i, bus_id) in bus_ids.iter().enumerate() {
        let slice_to_match = &bus_ids[0..i+1];
        while !check_time(tstamp, slice_to_match) {
            tstamp += step;
        }
        // increase step by the least common multiple to skip unecessary checks
        step = lcm(step, *bus_id);
    }

    println!("{:?}", tstamp);

    Ok(())
}
