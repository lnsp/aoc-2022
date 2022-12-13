use std::cmp::max;
use std::io;

fn main() -> io::Result<()> {
    // 1st step: build map of trees
    let mut levels: Vec<Vec<usize>> = Vec::new();
    let mut buf = String::new();
    while io::stdin().read_line(&mut buf)? > 0 {
        levels.push(
            buf.trim()
                .as_bytes()
                .iter()
                .map(|c| 1 + *c as usize - '0' as usize)
                .collect(),
        );
        buf.clear();
    }

    println!("{}", count_visible(&levels));
    println!("{}", highest_score(&levels));

    Ok(())
}

fn highest_score(levels: &Vec<Vec<usize>>) -> usize {
    // 2nd step: build visibility index
    let (n, m) = (levels.len(), levels[0].len());
    let mut scores = vec![vec![1 as usize; m]; n];

    // 3rd step: determine for viewing from top, left, right and bottom
    for i in 0..n {
        let mut view_distance = [0; 100];
        for j in 0..m {
            scores[i][j] *= max(1, j - view_distance[levels[i][j]]);
            for k in 0..=levels[i][j] {
                view_distance[k as usize] = j
            }
        }
        let mut view_distance = [m-1; 100];
        for j in (0..m).rev() {
            scores[i][j] *= max(1, view_distance[levels[i][j]] - j);
            for k in 0..=levels[i][j] {
                view_distance[k as usize] = j
            }
        }
    }


    for j in 0..m {
        let mut view_distance = [0; 100];
        for i in 0..n {
            scores[i][j] *= max(1, i - view_distance[levels[i][j]]);
            for k in 0..=levels[i][j] {
                view_distance[k as usize] = i
            }
        }
        let mut view_distance = [n-1; 100];
        for i in (0..n).rev() {
            scores[i][j] *= max(1, view_distance[levels[i][j]] - i);
            for k in 0..=levels[i][j] {
                view_distance[k as usize] = i
            }
        }
    }

    // 4th step: return max in scores
    *scores[1..n-1]
        .iter()
        .map(|s| s[1..m-1].iter().max().unwrap())
        .max()
        .unwrap()
}

fn count_visible(levels: &Vec<Vec<usize>>) -> i32 {
    // 2nd step: build visibility index
    let (n, m) = (levels.len(), levels[0].len());
    let mut visible = vec![vec![false; m]; n];

    // 3rd step: determine for viewing from top, left, right and bottom
    for i in 0..n {
        let mut highest_level = 0;
        for j in 0..m {
            visible[i][j] |= highest_level < levels[i][j];
            highest_level = max(highest_level, levels[i][j]);
        }
        let mut highest_level = 0;
        for j in (0..m).rev() {
            visible[i][j] |= highest_level < levels[i][j];
            highest_level = max(highest_level, levels[i][j]);
        }
    }
    for j in 0..m {
        let mut highest_level = 0;
        for i in 0..n {
            visible[i][j] |= highest_level < levels[i][j];
            highest_level = max(highest_level, levels[i][j]);
        }
        let mut highest_level = 0;
        for i in (0..n).rev() {
            visible[i][j] |= highest_level < levels[i][j];
            highest_level = max(highest_level, levels[i][j]);
        }
    }

    visible
        .iter()
        .map(|r| {
            r.iter()
                .map(|b| match b {
                    false => 0,
                    true => 1,
                })
                .sum::<i32>()
        })
        .sum::<i32>()
}
