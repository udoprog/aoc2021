use std::collections::{HashMap, VecDeque};

use anyhow::Result;
use aoc::Parser;

fn main() -> Result<()> {
    let input = aoc::load("09.txt")?;
    let mut p = Parser::new(&input);

    let mut y = 0isize;
    let mut map = HashMap::new();

    while let Some(line) = p.next_line().map(Parser::into_str) {
        for (x, c) in line
            .char_indices()
            .flat_map(|(x, c)| Some((x, c.to_digit(10)?)))
        {
            map.insert((x as isize, y), c);
        }

        y += 1;
    }

    let mut part1 = 0;

    for (&p, &current) in &map {
        if neigh(p).all(|n| map.get(&n).map(|&at| at > current).unwrap_or(true)) {
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

        while let Some(p) = queue.pop_front() {
            for n in neigh(p) {
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

fn neigh(p: (isize, isize)) -> impl Iterator<Item = (isize, isize)> {
    [
        (p.0 + 1, p.1),
        (p.0 - 1, p.1),
        (p.0, p.1 + 1),
        (p.0, p.1 - 1),
    ]
    .into_iter()
}
