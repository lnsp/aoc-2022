use std::{
    cmp::{max, min},
    io,
};

use itertools::Itertools;

#[derive(Clone, Copy)]
enum Tile {
    Air,
    Rock,
    Sand,
}

fn main() -> io::Result<()> {
    let mut tiles = vec![vec![Tile::Air; 1000]; 1000];

    let mut buf = String::new();
    let mut floor = 0;

    while io::stdin().read_line(&mut buf)? > 0 {
        let path: Vec<(usize, usize)> = buf
            .trim()
            .split(" -> ")
            .map(|s| {
                s.split(",")
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect();

        for i in 0..path.len() - 1 {
            for j in min(path[i].0, path[i + 1].0)..=max(path[i].0, path[i + 1].0) {
                for k in min(path[i].1, path[i + 1].1)..=max(path[i].1, path[i + 1].1) {
                    tiles[k][j] = Tile::Rock;

                    floor = max(floor, k + 2);
                }
            }
        }

        buf.clear();
    }

    // 2nd step: simulate sand
    let mut iteration = 0;
    loop {
        let (mut sx, mut sy) = (500, 0);

        match tiles[sy][sx] {
            Tile::Sand => break,
            _ => ()
        };

        loop {
            if sy+1 == floor {
                tiles[sy][sx] = Tile::Sand;
                iteration += 1;
                break;
            }
            // Either sand falls downard directly
            match tiles[sy + 1][sx] {
                Tile::Air => sy += 1,
                _ => match tiles[sy + 1][sx - 1] {
                    Tile::Air => (sx, sy) = (sx - 1, sy + 1),
                    _ => match tiles[sy + 1][sx + 1] {
                        Tile::Air => (sx, sy) = (sx + 1, sy + 1),
                        _ => {
                            tiles[sy][sx] = Tile::Sand;
                            iteration += 1;
                            break;
                        }
                    },
                },
            };
        };
    }

    println!("{}", iteration);

    Ok(())
}
