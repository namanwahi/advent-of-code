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
    let points: Vec<Vec<u32>> = reader.lines()
        .map(|line| line.unwrap().chars().map(|c| (c == '#') as u32).collect::<Vec<_>>())
        .collect();

    let width = points[0].len();
    let depth = points.len();

    let res = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|slope| {
            let mut crashes = 0;
            let mut pos = (0, 0);
            while pos.1 < depth {
                crashes += points[pos.1][pos.0];
                pos.0 = (pos.0 + slope.0) % width;
                pos.1 += slope.1;
            };
            crashes as u64
        }).fold(1, |acc, elem| acc * elem);

    println!("{}", res);
    Ok(())
}
