use std::collections::HashMap;

use anyhow::{anyhow, Result};
use aoc::Parser;

struct Sub {
    m: [char; 2],
    to: char,
}

fn main() -> Result<()> {
    let input = aoc::load("14.txt")?;
    let mut p = Parser::new(&input);

    let template = p.line()?.into_str().chars().collect::<Vec<_>>();
    p.line()?;

    let mut rules = HashMap::new();

    while let Some(sub) = p.next_line().map(Parser::into_str).and_then(parse) {
        rules.insert(sub.m, sub.to);
    }

    assert_eq!(solve2(&template, 10, &rules)?, 2947);
    assert_eq!(solve2(&template, 40, &rules)?, 3232426226464);
    Ok(())
}

fn parse(s: &str) -> Option<Sub> {
    let (m, to) = s.split_once(" -> ")?;

    let mut m = m.chars();
    let a = m.next()?;
    let b = m.next()?;

    let mut to = to.chars();
    let to = to.next()?;

    Some(Sub { m: [a, b], to })
}

fn solve2(chain: &[char], n: usize, rules: &HashMap<[char; 2], char>) -> Result<usize> {
    let mut counts = Counts::default();
    let mut memo = HashMap::new();

    for w in chain.windows(2) {
        if let &[a, b] = w {
            merge_to(&mut counts, inner(a, b, rules, n, &mut memo));
        }
    }

    if let Some(c) = chain.last() {
        counts[as_index(*c)] += 1;
    }

    let mut values = counts.into_iter().filter(|n| *n != 0).collect::<Vec<_>>();
    values.sort();

    let first = *values.first().ok_or_else(|| anyhow!("missing first"))?;
    let last = *values.last().ok_or_else(|| anyhow!("missing last"))?;
    return Ok(last - first);

    type Counts = [usize; 26];

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Key(char, char, usize);

    fn inner(
        a: char,
        b: char,
        rules: &HashMap<[char; 2], char>,
        n: usize,
        memo: &mut HashMap<Key, Counts>,
    ) -> Counts {
        if n == 0 {
            let mut counts = Counts::default();
            counts[as_index(a)] = 1;
            return counts;
        }

        let key = Key(a, b, n);

        if let Some(counts) = memo.get(&key) {
            return *counts;
        }

        let counts = if let Some(c) = rules.get(&[a, b]).copied() {
            let mut counts = inner(a, c, rules, n - 1, memo);
            merge_to(&mut counts, inner(c, b, rules, n - 1, memo));
            counts
        } else {
            let mut counts = Counts::default();
            counts[as_index(a)] = 1;
            counts
        };

        memo.insert(key, counts);
        counts
    }

    fn merge_to(to: &mut Counts, from: Counts) {
        for (n, v) in from.into_iter().enumerate() {
            to[n] += v;
        }
    }

    fn as_index(c: char) -> usize {
        c as usize - 'A' as usize
    }
}
