use std::collections::HashMap;

use anyhow::Result;
use aoc::Parser;

#[derive(Debug, Clone, Copy)]
struct Line {
    from: (isize, isize),
    to: (isize, isize),
}

fn main() -> Result<()> {
    let input = aoc::load("05.txt")?;
    let mut p = Parser::new(&input);

    let mut lines = Vec::new();

    while let Some(line) = p.next_line().and_then(parse) {
        lines.push(line);
    }

    let count = solve(true, &lines);
    assert_eq!(count, 7269);

    let count = solve(false, &lines);
    assert_eq!(count, 21140);
    Ok(())
}

fn parse(line: &str) -> Option<Line> {
    let (first, second) = line.split_once(" -> ")?;
    let (fx, fy) = first.split_once(',')?;
    let (tx, ty) = second.split_once(',')?;

    let fx = str::parse(fx).ok()?;
    let fy = str::parse(fy).ok()?;
    let tx = str::parse(tx).ok()?;
    let ty = str::parse(ty).ok()?;

    Some(Line {
        from: (fx, fy),
        to: (tx, ty),
    })
}

fn solve(part1: bool, lines: &[Line]) -> usize {
    let mut map = HashMap::<_, u32>::new();

    for line in lines {
        let mut from = line.from;
        let mut to = line.to;

        let dx = (to.0 - from.0).signum();
        let dy = (to.1 - from.1).signum();

        if part1 && dx != 0 && dy != 0 {
            continue;
        }

        to.0 += dx;
        to.1 += dy;

        while from != to {
            *map.entry(from).or_default() += 1;

            from.0 += dx;
            from.1 += dy;
        }
    }

    map.values().filter(|n| **n >= 2).count()
}
