use std::fs::File;
use std::io::{self, BufRead, ErrorKind};
use std::error::Error;
use std::collections::{HashSet, HashMap};
use std::ops::Index;
use std::cmp;
use std::iter::{self,Chain};
use std::slice::Iter;
use std::fmt;

fn get_seat(seat_code: &str) -> (u32, u32) {
    let seat = seat_code.chars()
        .fold((0, 127, 0, 7), |(r_l, r_h, c_l, c_h), c| {
            let mid_r: u32 = (r_l + r_h) / 2;
            let mid_c: u32 = (c_l + c_h) / 2; 
            match c {
                'B' => (mid_r + 1, r_h, c_l, c_h),
                'F' => (r_l, mid_r, c_l, c_h),
                'R' => (r_l, r_h, mid_c + 1, c_h),
                'L' => (r_l, r_h, c_l, mid_c),
                _ => panic!("Invalid char"),
            }});
    (seat.0, seat.2)
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("inputs.txt")?;
    let reader = io::BufReader::new(f);

    let seats: Vec<_> = reader.lines()
        .map(|seat| get_seat(&(seat.unwrap())))
        .map(|(row, col)| row * 8 + col).collect();

    let (min, max, sum) = seats.iter()
        .fold((u32::MAX, 0, 0), |(min, max, sum), code| (cmp::min(min, *code), cmp::max(max, *code), sum + code));

    println!("Part 1 anwser {}", max);

    // calculate expected sum of all seats using the formula for the sum of integers (n * (n + 1) / 2)
    let sum_all_seats = ((max * (max + 1)) / 2 - ((min - 1) * min) / 2);
    let missing_seat = sum_all_seats - sum;
    
    println!("Part two answer {}", missing_seat);
    Ok(())
}
