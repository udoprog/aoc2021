use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use anyhow::Result;
use aoc::LineParser;

fn main() -> Result<()> {
    let input = aoc::load("15.txt")?;
    let mut p = LineParser::new(&input);

    let mut map = HashMap::new();

    let mut w = 0;
    let mut h = 0;

    let mut y = 0isize;

    while let Some(line) = p.next() {
        for (x, c) in line.chars().enumerate() {
            let x = x as isize;
            let mut s = [0, 0, 0, 0];
            let c = str::parse::<usize>(c.encode_utf8(&mut s))?;
            map.insert((x, y), c);
            w = isize::max(w, x + 1);
            h = isize::max(h, y + 1);
        }

        y += 1;
    }

    assert_eq!(solve(&map, (w - 1, h - 1)), Some(609));
    grow(&mut map, 5, w, h);
    assert_eq!(solve(&map, (w * 5 - 1, h * 5 - 1)), Some(2925));
    Ok(())
}

fn neigh((x, y): (isize, isize)) -> impl Iterator<Item = (isize, isize)> {
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].into_iter()
}

fn solve(map: &HashMap<(isize, isize), usize>, target: (isize, isize)) -> Option<usize> {
    let mut queue = BinaryHeap::new();
    let mut costs = HashMap::new();

    queue.push(Reverse((0, (0, 0))));

    while let Some(Reverse((cost, cur))) = queue.pop() {
        if cur == target {
            return Some(cost);
        }

        for next in neigh(cur) {
            let c = match map.get(&next).copied() {
                Some(c) => cost + c,
                None => continue,
            };

            if let Some(existing) = costs.get(&next).copied() {
                if existing <= c {
                    continue;
                }
            }

            costs.insert(next, c);
            queue.push(Reverse((c, next)));
        }
    }

    None
}

fn grow(map: &mut HashMap<(isize, isize), usize>, factor: isize, w: isize, h: isize) {
    let keys = map.keys().copied().collect::<Vec<_>>();

    for x in 0..factor {
        for y in 0..factor {
            let manhattan = (x + y) as usize;

            if manhattan == 0 {
                continue;
            }

            for key in &keys {
                if let Some(c) = map.get(&key).copied() {
                    let c = (c - 1 + manhattan) % 9 + 1;
                    let new = ((w * x) + key.0, (h * y) + key.1);
                    map.insert(new, c);
                }
            }
        }
    }
}
