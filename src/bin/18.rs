use anyhow::Result;
use aoc::Parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Entry {
    level: usize,
    value: u32,
}

#[derive(Default, Clone)]
struct Snail {
    data: Vec<Entry>,
}

impl Snail {
    fn add(&mut self, mut other: Self) {
        if self.data.is_empty() {
            self.data = other.data;
        } else {
            self.data.append(&mut other.data);

            for e in &mut self.data {
                e.level += 1;
            }
        }

        process(&mut self.data);
    }

    /// Calculate the magnitude of a number of entries.
    fn magnitude(&self) -> u32 {
        let mut data = self.data.to_vec();

        while data.len() != 1 {
            let mut n = 0;

            while n != data.len() && data.len() != 1 {
                if data[n].level == data[n + 1].level {
                    data[n].value = data[n].value * 3 + data[n + 1].value * 2;
                    data[n].level -= 1;
                    data.remove(n + 1);
                    n = n.saturating_sub(1);
                    continue;
                }

                n += 1;
            }
        }

        assert_eq!(data[0].level, 0);
        data[0].value
    }
}

fn main() -> Result<()> {
    let input = aoc::load("18.txt")?;
    let mut p = Parser::new(&input);

    let mut full = Snail::default();
    let mut all = Vec::new();

    while let Some(snail) = p.next_line().and_then(parse) {
        all.push(snail.clone());
        full.add(snail);
    }

    let part1 = full.magnitude();

    let mut part2 = u32::MIN;

    for a in 0..all.len() {
        for b in (a + 1)..all.len() {
            {
                let mut a = all[a].clone();
                a.add(all[b].clone());
                part2 = u32::max(a.magnitude(), part2);
            }

            {
                let mut b = all[b].clone();
                b.add(all[a].clone());
                part2 = u32::max(b.magnitude(), part2);
            }
        }
    }

    assert_eq!(part1, 4124);
    assert_eq!(part2, 4673);
    Ok(())
}

fn parse(p: Parser<'_>) -> Option<Snail> {
    let s = p.into_str();

    let mut data = Vec::new();
    let mut level = 0;

    for c in s.chars() {
        match c {
            '[' => {
                level += 1;
            }
            ']' => {
                level -= 1;
            }
            c => {
                if let Some(value) = c.to_digit(10) {
                    data.push(Entry { level, value })
                };
            }
        }
    }

    Some(Snail { data })
}

fn process(num: &mut Vec<Entry>) {
    let mut n = 0;

    while n != num.len() {
        // Special test here since length might be modified while exploding.
        while n != num.len() {
            explode(num, n);
            n += 1;
        }

        for i in 0..num.len() {
            if split(num, i) {
                // Only need to re-process from the number that was split since
                // the collection was only modified from this point.
                n = i;
                break;
            }
        }
    }
}

fn explode(num: &mut Vec<Entry>, n: usize) {
    let a = match num.get(n).copied() {
        Some(e) if e.level > 4 => e,
        _ => return,
    };

    let b = match num.get(n + 1).copied() {
        Some(e) if e.level == a.level => e,
        _ => return,
    };

    // NB: there is a before index.
    if let Some(e) = n.checked_sub(1).and_then(|n| num.get_mut(n)) {
        e.value += a.value;
    }

    if let Some(e) = num.get_mut(n) {
        e.value = 0;
        e.level -= 1;
        // No input provides a number that is nested deeper than this.
        assert_eq!(e.level, 4);
    }

    if let Some(next) = num.get_mut(n + 2) {
        next.value += b.value;
    }

    num.remove(n + 1);
}

/// Split this number.
fn split(num: &mut Vec<Entry>, n: usize) -> bool {
    let e = match num.get_mut(n) {
        Some(e) if e.value > 9 => e,
        _ => return false,
    };

    let right = (e.value + 1) / 2;
    let level = e.level + 1;

    e.level = level;
    e.value = e.value / 2;

    num.insert(
        n + 1,
        Entry {
            level,
            value: right,
        },
    );

    true
}
