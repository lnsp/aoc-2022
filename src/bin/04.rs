use itertools::Itertools;
use std::{cmp::max, io};

fn main() -> io::Result<()> {
    let count = io::stdin()
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| {
            l.split(",")
                .map(|s| {
                    s.split("-")
                        .filter_map(|s| s.parse::<i32>().ok())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple::<((i32, i32), (i32, i32))>()
                .unwrap()
        })
        .filter(|(a, b)| {
            max(a.1 - b.1, b.1 - a.1) <= max(a.1 - a.0, b.1 - b.0)
                && max(a.0 - b.0, b.0 - a.0) <= max(a.1 - a.0, b.1 - b.0)
        })
        .count();

    println!("{}", count);

    Ok(())
}
