use anyhow::Result;
use aoc::{ParseError, Parseable};
use std::{fmt, ops::Index};

const DIGITS: usize = 16;

#[derive(Debug, Clone, Copy)]
struct Bits(usize, [bool; DIGITS]);

impl Bits {
    /// Get the number of bits.
    fn len(&self) -> usize {
        self.0
    }

    /// Coerce into a u64.
    fn into_u64(self) -> u64 {
        let mut out = 0u64;

        for (n, d) in self.1.into_iter().take(self.0).enumerate() {
            out += (if d { 1 } else { 0 }) << (self.0 - 1 - n) as u64;
        }

        out
    }

    fn iter(&self) -> impl Iterator<Item = bool> {
        self.1.into_iter().take(self.0)
    }
}

impl Index<usize> for Bits {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.0, "length overflow");
        &self.1[index]
    }
}

impl fmt::Display for Bits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in self.iter() {
            if b { "1" } else { "0" }.fmt(f)?;
        }

        Ok(())
    }
}

impl FromIterator<bool> for Bits {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        let mut len = 0;
        let mut init = [false; DIGITS];

        for (b, c) in init.iter_mut().zip(iter) {
            len += 1;
            *b = c;
        }

        Bits(len, init)
    }
}

impl Parseable for Bits {
    fn parse(p: &mut aoc::Parser<'_>) -> Result<Self, ParseError> {
        let mut init = [false; DIGITS];

        let line = p.next()?;

        for (b, c) in init.iter_mut().zip(line.chars()) {
            *b = matches!(c, '1');
        }

        Ok(Bits(line.len(), init))
    }
}

fn main() -> Result<()> {
    let input = aoc::load("03.txt")?;
    let lines = aoc::lines::<Bits>(input)?;

    // Positive numbers means more 1's, negative numbers means more 0's.
    let mut counts = Vec::new();

    for bits in &lines {
        counts.resize(bits.len(), 0);

        for (count, b) in counts.iter_mut().zip(bits.iter()) {
            *count += if b { 1 } else { -1 };
        }
    }

    let mut gamma = 0u64;
    let mut epsilon = 0u64;

    let digest: Bits = counts
        .into_iter()
        .map(|d| if d >= 0 { true } else { false })
        .collect();

    for (n, d) in digest.iter().enumerate() {
        gamma += (if d { 1 } else { 0 }) << (digest.len() - 1 - n) as u128;
        epsilon += (if d { 0 } else { 1 }) << (digest.len() - 1 - n) as u128;
    }

    assert_eq!(gamma * epsilon, 4103154);

    // let a = find(&lines, &digest);
    let a = find(&lines, true);
    let b = find(&lines, false);

    if let (Some(a), Some(b)) = (a, b) {
        assert_eq!(a.into_u64() * b.into_u64(), 4245351);
    } else {
        panic!("no answer found");
    }

    Ok(())
}

fn find<'a>(lines: &'a [Bits], most: bool) -> Option<Bits> {
    let mut lines = lines.to_vec();

    for n in 0.. {
        let expect = lines.iter().map(|b| if b[n] { 1 } else { -1 }).sum::<i32>();

        let expect = if most { expect >= 0 } else { expect < 0 };

        lines.retain(|b| b[n] == expect);

        if lines.len() == 1 {
            return Some(lines[0]);
        }

        if lines.is_empty() {
            break;
        }
    }

    None
}
