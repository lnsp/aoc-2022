use std::{io};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let priority = | c: u8 | {
        if (c >= 'a' as u8) && (c <= 'z' as u8) {
            1 + c - 'a' as u8
        } else {
            27 + c - 'A' as u8
        }
    };
    let mut sum = 0;
    for line in io::stdin().lines() {
        let line_result = line?;
        let bytes = line_result.trim().as_bytes();
        let size = bytes.len() / 2;
        let mut first = HashSet::<u8>::new();
        let mut second = HashSet::<u8>::new();

        bytes[..size].iter().for_each(| x | { _ = first.insert(*x)} );
        bytes[size..].iter().for_each(| x | { _ = second.insert(*x)} );

        sum += first.intersection(&second).map(|x| priority(*x) as i32).sum::<i32>();
    }

    println!("{}", sum);

    Ok(())
}