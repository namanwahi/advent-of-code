use std::collections::{HashSet, HashMap};

fn main() {
    let starting_nums = vec![14, 8, 16, 0, 1, 17];
    let mut last_seen: HashMap<usize, (Option<usize>, Option<usize>)> = HashMap::new();

    for (i, starting_num) in starting_nums.iter().enumerate() {
        last_seen.insert(*starting_num, (Some(i), None));
    }

    let mut last_num = *starting_nums.last().unwrap();

    for round in (starting_nums.len()..30000000) {
        last_num = match last_seen.get(&last_num).unwrap() {
            (Some(i), None) => 0,
            (Some(i), Some(j)) => j - i,
            _ => panic!("halp"),
        };

        match last_seen.get(&last_num) {
            None => {
                last_seen.insert(last_num, (Some(round), None))
            },
            Some((Some(i), None)) => {
                last_seen.insert(last_num, (Some(*i), Some(round)))
            },
            Some((Some(i), Some(j))) => {
                last_seen.insert(last_num, (Some(*j), Some(round)))
            },
            _ => panic!("Halp"),
        };
    }

    println!("{:?}", last_num);
}
