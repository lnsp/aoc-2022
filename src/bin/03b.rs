#![feature(iter_array_chunks)]

use std::collections::HashSet;
use std::io;


fn main() -> io::Result<()> {
    let priority = | c: u8 | {
        if (c >= 'a' as u8) && (c <= 'z' as u8) {
            1 + c - 'a' as u8
        } else {
            27 + c - 'A' as u8
        }
    };
    let total_sum: i32 = io::stdin()
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| HashSet::<u8>::from_iter(l.trim().as_bytes().iter().cloned()))
        .array_chunks::<3>()
        .map(|c| {
            let i1 = &c[0] & &c[1];
            let i2 = &i1 & &c[2];
            i2.iter().next().and_then(|x| Some(*x))
        })
        .filter_map(|x| x)
        .fold(0, |a, b| a + priority(b) as i32);

    println!("{}", total_sum);

    Ok(())
}
