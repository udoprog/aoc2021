use std::collections::{HashMap, VecDeque};

use anyhow::Result;
use aoc::LineParser;

fn main() -> Result<()> {
    let input = aoc::load("09.txt")?;
    let mut p = LineParser::new(input);

    let mut y = 0isize;
    let mut map = HashMap::new();

    while let Some(line) = p.try_line()? {
        for (x, c) in line.char_indices() {
            let mut buf = [0, 0, 0, 0];
            let s = c.encode_utf8(&mut buf);
            map.insert((x as isize, y), str::parse::<u32>(s)?);
        }

        y += 1;
    }

    let mut part1 = 0;

    for (&(x, y), &current) in &map {
        if neigh(x, y).all(|n| map.get(&n).map(|&at| at > current).unwrap_or(true)) {
            part1 += current + 1;
        }
    }

    let mut basins = Vec::new();

    while let Some(n) = map.keys().next().copied() {
        if matches!(map.remove(&n), Some(9) | None) {
            continue;
        }

        let mut size = 1;

        let mut queue = VecDeque::new();
        queue.push_back(n);

        while let Some((x, y)) = queue.pop_front() {
            for n in neigh(x, y) {
                if !matches!(map.remove(&n), Some(9) | None) {
                    size += 1;
                    queue.push_back(n);
                }
            }
        }

        basins.push(size);
    }

    basins.sort_unstable();

    let part2 = basins
        .into_iter()
        .rev()
        .take(3)
        .try_fold(1, u32::checked_mul);

    assert_eq!(part1, 631);
    assert_eq!(part2, Some(821560));
    Ok(())
}

fn neigh(x: isize, y: isize) -> impl Iterator<Item = (isize, isize)> {
    [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter()
}
