use std::fs::File;
use std::str;
use std::io::{self, BufRead};
use std::error::Error;
use std::cmp::{max, min};

#[macro_use]
extern crate ndarray;
use ndarray::Array2;

fn round(arr: &mut Array2<char>) -> bool {
    let mut flipped: Vec<(usize, usize)> = Vec::new();
    for i in 0..arr.shape()[0] {
        for j in 0..arr.shape()[1] {
            /* PART 1
            let min_row = if i >= 1 { i - 1 } else { 0 };
            let min_col = if j >= 1 { j - 1 } else { 0 };
            let max_row = min(i + 1, arr.shape()[0] - 1);
            let max_col = min(j + 1, arr.shape()[1] - 1);
            let region = arr.slice(s![min_row..max_row + 1, min_col..max_col + 1]);
            let flip: bool = match arr[[i, j]] {
                'L' => region.iter().filter(|&&c| c == '#').count() == 0,
                '#' => region.iter().filter(|&&c| c == '#').count() >= 5,
                _ => false,
            };*/

            // PART 2
            let seen_occupied = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)].into_iter()
                .map(|(x_dir, y_dir)| {
                    let mut steps = 0;
                    let res = loop {
                        steps += 1;
                        let steps = steps as i64;
                        let new_i = (i as i64) + (x_dir * steps);
                        let new_j = (j as i64) + (y_dir * steps);

                        if new_i < 0 || new_i >= (arr.shape()[0] as i64) || new_j < 0 || new_j >= (arr.shape()[1] as i64) {
                            break false;
                        }

                        let potential_seat = arr[[new_i as usize, new_j as usize]];
                        if potential_seat == '#' {
                            break true;
                        } else if potential_seat == 'L' {
                            break false;
                        }
                    };
                    res
                }).filter(|&b| b).count();

            let flip: bool = match arr[[i, j]] {
                'L' => seen_occupied == 0,
                '#' => seen_occupied >= 5,
                _ => false,
            };

            if flip {
                flipped.push((i, j));
            }
        }
    }

    if flipped.len() == 0 {
        return true
    }

    for (i, j) in flipped.into_iter() {
        arr[[i, j]] = if arr[[i, j]] == '#' { 'L' } else if arr[[i, j]] == 'L' { '#' } else { panic!("HALP") };
    }

    false
}

fn main() -> Result<(), Box<dyn Error>>{
    let f = File::open("input.txt")?;
    let reader = io::BufReader::new(f);
    let mut data = Vec::new();
    let mut nrows = 0;
    for line in reader.lines() {
        data.extend_from_slice(&line.unwrap().chars().collect::<Vec<_>>());
        nrows += 1;
    }

    let mut arr = Array2::from_shape_vec((nrows, data.len() / nrows), data)?;
    while !round(&mut arr) { }

    println!("{:?}", arr.iter().filter(|&&c| c == '#').count());

    Ok(())
}
