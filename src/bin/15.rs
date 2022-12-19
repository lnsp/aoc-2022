use std::{cmp::max, io};

use text_io::scan;

fn main() -> io::Result<()> {
    let mut buf = String::new();

    let mut sensors = Vec::new();
    while io::stdin().read_line(&mut buf)? > 0 {
        let (sx, sy, bx, by): (i32, i32, i32, i32);
        scan!(buf.trim().bytes() => "Sensor at x={}, y={}: closest beacon is at x={}, y={}", sx, sy, bx, by);

        sensors.push((sx, sy, (bx.abs_diff(sx) + by.abs_diff(sy)) as i32));
        buf.clear();
    }

    let mut ranges = Vec::new();
    for target_row in 0..4000000 {
        for (sx, sy, mhd) in &sensors {
            let distance = mhd - sy.abs_diff(target_row) as i32;

            if distance < 0 {
                continue;
            }

            ranges.push((sx - distance, sx + distance));
        }

        // Sort ranges by start
        ranges.sort();

        // Merge them
        let mut dissect_ranges = Vec::new();
        for (l, r) in &ranges {
            match dissect_ranges.last_mut() {
                None => dissect_ranges.push((l, r)),
                Some((_, er)) => {
                    // Check for overlap
                    if *l <= **er + 1 {
                        *er = max(r, *er);
                    } else {
                        dissect_ranges.push((l, r));
                    }
                }
            };
        }

        if dissect_ranges.len() > 1 {
            println!(
                "{}",
                target_row as i64 + (dissect_ranges[0].1 + 1) as i64 * 4000000
            );
        }

        ranges.clear();
    }

    Ok(())
}
