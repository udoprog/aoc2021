use anyhow::Result;
use aoc::Parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Entry {
    level: usize,
    value: u32,
}

fn main() -> Result<()> {
    let input = aoc::load("18.txt")?;
    let mut p = Parser::new(&input);

    let mut base = Vec::<Entry>::new();

    while let Some(mut num) = p.next_line().and_then(parse) {
        if base.is_empty() {
            base = num;
        } else {
            base.append(&mut num);

            for e in &mut base {
                e.level += 1;
            }
        }

        process(&mut base);
    }

    let part1 = magnitude(&base);
    assert_eq!(part1, 4124);
    Ok(())
}

fn parse(p: Parser<'_>) -> Option<Vec<Entry>> {
    let s = p.into_str();

    let mut output = Vec::new();
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
                    output.push(Entry { level, value })
                };
            }
        }
    }

    Some(output)
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

/// Calculate the magnitude of a number of entries.
fn magnitude(num: &[Entry]) -> u32 {
    let mut num = num.to_vec();

    while num.len() != 1 {
        let mut n = 0;

        while n != num.len() && num.len() != 1 {
            if num[n].level == num[n + 1].level {
                num[n].value = num[n].value * 3 + num[n + 1].value * 2;
                num[n].level -= 1;
                num.remove(n + 1);
                n = n.saturating_sub(1);
                continue;
            }

            n += 1;
        }
    }

    assert_eq!(num[0].level, 0);
    num[0].value
}
