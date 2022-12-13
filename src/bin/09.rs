use std::{collections::HashSet, io};

fn main() -> io::Result<()> {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut knots = [(0, 0); 10];

    let mut buf = String::new();
    while io::stdin().read_line(&mut buf)? > 0 {
        let args: Vec<&str> = buf.trim().split_whitespace().collect();
        visited.insert(knots[9]);
        for _ in 0..args[1].parse::<i32>().unwrap() {
            let head_knot = &mut knots[0];
            *head_knot = match args[0] {
                "U" => (head_knot.0 - 1, head_knot.1),
                "D" => (head_knot.0 + 1, head_knot.1),
                "R" => (head_knot.0, head_knot.1 + 1),
                "L" => (head_knot.0, head_knot.1 - 1),
                _ => panic!("unknown direction {}", args[0]),
            };
            // update tails based on diff
            for i in 1..10 {
                let (head_part, tail_part) = knots.split_at_mut(i);
                let (head_knot, tail_knot) = (&mut head_part[i - 1], &mut tail_part[0]);

                let distance = ((head_knot.0 - tail_knot.0), (head_knot.1 - tail_knot.1));
                *tail_knot = match (distance.0.abs(), distance.1.abs()) {
                    (0, 0) | (0, 1) | (1, 0) | (1, 1) => *tail_knot,
                    _ => (
                        tail_knot.0 + distance.0.signum(),
                        tail_knot.1 + distance.1.signum(),
                    ),
                };
            }
            visited.insert(knots[9]);
        }
        buf.clear();
    }

    println!("{}", visited.len());

    Ok(())
}
