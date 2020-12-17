use std::io::{self, BufRead};
use std::error::Error;
use std::fs::File;
use std::collections::{HashMap, HashSet};

type Point = (i64, i64, i64, i64);

fn add(p1: Point, p2: Point) -> Point {
    (p1.0 + p2.0, p1.1 + p2.1, p1.2 + p2.2, p1.3 + p2.3)
}

fn main() -> Result<(), Box<dyn Error>>{
    let f = File::open("inputs.txt")?;
    let lines = io::BufReader::new(f).lines().map(|line| line.unwrap());

    let mut active = HashSet::new();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active.insert((x as i64, y as i64, 0, 0));   
            }
        }
    }

    let mut directions: Vec<Point> = vec![];
    for x_dir in (-1..2) {
        for y_dir in (-1..2) {
            for z_dir in (-1..2) {
                for w_dir in (-1..2) {
                    let direction = (x_dir, y_dir, z_dir, w_dir);
                    if direction != (0, 0, 0, 0) {
                        directions.push(direction);
                    }
                }
            }
        }
    }

    for r in (0..6) {
        let candidate_points: HashSet<Point> = active.iter()
            .flat_map(|&p| directions.iter().map(move |&d| add(d, p)))
            .chain(active.iter().map(|p| *p))
            .filter(|&p| {
                let active_neighbours = directions.iter().map(|&d| add(p, d)).filter(|p| active.contains(&p)).count();
                if active.contains(&p) {
                    active_neighbours == 3 || active_neighbours == 2
                } else {
                    active_neighbours == 3
                }
            })
            .collect();

        active = candidate_points;
        println!("Round {} - active {}", r + 1, active.len());
    }

    Ok(())
}
