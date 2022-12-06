use std::{io, ops::BitOr};

fn get_msg_start(text: &[u8], n: usize) -> i32 {
    let mut last_n = vec![0; n];

    text.iter().enumerate().fold(-1, |b, (i, c)| {
        if b != -1 {
            b
        } else {
            last_n[i % n] = 1 << (*c as u8 - 'a' as u8);
            if last_n.iter().fold(0 as u32, |a, b| a | b).count_ones() == n as u32 {
                i as i32 + 1
            } else {
                b
            }
        }
    })
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    let content = buf.trim().as_bytes();

    println!("{}", get_msg_start(content, 14));

    Ok(())
}
