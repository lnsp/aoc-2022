use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let (mut last_cycle, mut last_value): (i32, i32) = (1, 1);
    let mut buf = String::new();
    let mut pixels = vec![vec![false; 40]; 6];
    while io::stdin().read_line(&mut buf)? > 0 {
        let args: Vec<&str> = buf.trim().split_whitespace().collect();

        let (next_cycle, next_value) = match args[0] {
            "noop" => (last_cycle + 1, last_value),
            "addx" => (last_cycle + 2, last_value + args[1].parse::<i32>().unwrap()),
            _ => panic!("unknown arg {}", args[0]),
        };

        for cycle in last_cycle..next_cycle {
            let cycle = cycle - 1;
            pixels[(cycle / 40) as usize][(cycle % 40) as usize] =
                ((cycle % 40) - 1..=(cycle % 40) + 1).contains(&last_value);
        }

        (last_cycle, last_value) = (next_cycle, next_value);

        buf.clear();
    }
    println!(
        "{}",
        pixels
            .iter()
            .map(|p| p
                .iter()
                .map(|x| match x {
                    false => ".",
                    true => "#",
                })
                .join(""))
            .join("\n")
    );

    Ok(())
}
