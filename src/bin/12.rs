use std::{collections::VecDeque, io};

fn main() -> io::Result<()> {
    // 1st step: scan map
    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut buf = String::new();
    while io::stdin().read_line(&mut buf)? > 0 {
        map.push(Vec::from(buf.trim().as_bytes()));
        buf.clear();
    }
    let (n, m) = (map.len() as i32, map.first().unwrap().len() as i32);

    // 2nd step: determine start and end
    let (mut ex, mut ey) = (0, 0);
    for i in 0..n {
        for j in 0..m {
            map[i as usize][j as usize] = match map[i as usize][j as usize] {
                83 => 0,
                69 => {
                    (ex, ey) = (j, i);
                    25
                }
                c => c - 97,
            }
        }
    }

    // 3rd step: find best path
    let mut positions: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let mut visited = vec![vec![false; m as usize]; n as usize];
    let directions: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    for i in 0..n {
        for j in 0..m {
            if map[i as usize][j as usize] == 0 {
                positions.push_back((i, j, 0));
            }
        }
    }

    let count = loop {
        let (i, j, c) = positions.pop_front().unwrap();
        if j == ex && i == ey {
            break c;
        }
        if visited[i as usize][j as usize] {
            continue;
        }
        visited[i as usize][j as usize] = true;

        let current_elevation = map[i as usize][j as usize];
        for d in directions {
            let (di, dj) = (i + d.0, j + d.1);
            if di < 0 || di >= n || dj < 0 || dj >= m {
                continue;
            }
            let destination_elevation = map[di as usize][dj as usize];
            if current_elevation < destination_elevation
                && destination_elevation - current_elevation > 1
            {
                continue;
            }

            positions.push_back((di, dj, c + 1));
        }
    };

    println!("{}", count);

    Ok(())
}
