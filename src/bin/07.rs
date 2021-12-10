use anyhow::Result;
use aoc::LineParser;

fn main() -> Result<()> {
    let input = aoc::load("07.txt")?;
    let line = LineParser::new(input).line()?;

    let mut pos = Vec::new();

    for d in line.split(',') {
        pos.push(str::parse::<i64>(d)?);
    }

    pos.sort();

    let mut p1 = i64::MAX;
    let mut p2 = i64::MAX;

    for n in pos[0]..pos[pos.len() - 1] {
        p1 = Ord::min(p1, part1(&pos, n as i64));
        p2 = Ord::min(p2, part2(&pos, n as i64));
    }

    assert_eq!(p1, 336701);
    assert_eq!(p2, 95167302);
    Ok(())
}

fn part1(input: &[i64], at: i64) -> i64 {
    let mut sum = 0;

    for n in input {
        sum += (at - *n).abs();
    }

    sum
}

fn part2(input: &[i64], at: i64) -> i64 {
    let mut sum = 0;

    for n in input {
        let n = (at - *n).abs();
        sum += (n * (n + 1)) / 2;
    }

    sum
}
