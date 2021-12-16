use anyhow::Result;
use aoc::Parser;

#[derive(Default)]
struct Extra {
    part1: usize,
}

struct Decoder<'a> {
    /// Bytes being decoded.
    bytes: &'a [u32],
    /// Bit cursor.
    pos: usize,
}

impl<'a> Decoder<'a> {
    /// Construct a new decoder.
    fn new(bytes: &'a [u32]) -> Self {
        Self { bytes, pos: 0 }
    }

    /// Decode the next u32.
    fn next_u32(&mut self, bits: usize) -> Option<u32> {
        assert!(bits <= 32);
        let mut num = 0u32;

        for _ in 0..bits {
            num <<= 1;
            num += self.next()?;
        }

        Some(num)
    }

    /// Get the next bit.
    fn next(&mut self) -> Option<u32> {
        let b = *self.bytes.get(self.pos / 32)?;
        let m = self.pos % 32;
        self.pos += 1;
        Some((b >> (32 - m - 1)) & 0b1)
    }
}

fn main() -> Result<()> {
    let input = aoc::load("16.txt")?;
    let line = Parser::new(&input).line()?.into_str();

    let mut buf = Vec::new();
    let mut i = 0;

    for n in line.chars().flat_map(|c| c.to_digit(16)) {
        if i % u32::BITS == 0 {
            buf.push(0);
        }

        if let Some(b) = buf.last_mut() {
            *b <<= 4;
            *b |= n;
        }

        i += 4;
    }

    if let Some(b) = buf.last_mut() {
        *b <<= u32::BITS - i % u32::BITS;
    }

    let mut d = Decoder::new(&buf);

    let mut output = Default::default();

    let part2 = decode(&mut d, &mut output);

    assert_eq!(output.part1, 852);
    assert_eq!(part2, Some(19348959966392));
    Ok(())
}

enum Limit {
    Count(usize),
    End(usize),
}

impl Limit {
    fn advance(&mut self, d: &mut Decoder<'_>) -> bool {
        match self {
            Limit::Count(count) if *count > 0 => {
                *count -= 1;
                true
            }
            Limit::End(end) if d.pos < *end => true,
            _ => false,
        }
    }
}

fn decode(d: &mut Decoder<'_>, e: &mut Extra) -> Option<u64> {
    let version = d.next_u32(3)?;
    let id = d.next_u32(3)?;

    e.part1 += version as usize;

    if id == 4 {
        let mut num = 0u64;

        while d.next()? == 1 {
            num = num.checked_shl(4)?;
            num = num.checked_add(d.next_u32(4)? as u64)?;
        }

        num = num.checked_shl(4)?;
        num = num.checked_add(d.next_u32(4)? as u64)?;
        return Some(num);
    }

    let mut l = if d.next()? == 1 {
        Limit::Count(d.next_u32(11)? as usize)
    } else {
        let n = d.next_u32(15)? as usize;
        Limit::End(d.pos.checked_add(n)?)
    };

    let (mut cur, op): (_, fn(_, _) -> _) = match id {
        0 => (0, u64::checked_add),
        1 => (1, u64::checked_mul),
        2 => (u64::MAX, |a, b| Some(u64::min(a, b))),
        3 => (u64::MIN, |a, b| Some(u64::max(a, b))),
        id => {
            let op = match id {
                5 => |a, b| a > b,
                6 => |a, b| a < b,
                7 => |a, b| a == b,
                _ => return None,
            };

            assert!(l.advance(d));
            let a = decode(d, e)?;
            assert!(l.advance(d));
            let b = decode(d, e)?;
            assert!(!l.advance(d));
            return Some(if op(a, b) { 1 } else { 0 });
        }
    };

    while l.advance(d) {
        cur = op(cur, decode(d, e)?)?;
    }

    Some(cur)
}
