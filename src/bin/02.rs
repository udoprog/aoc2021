use anyhow::anyhow;
use anyhow::Result;
use aoc::{ParseError, Parseable};

#[derive(Debug, Clone, Copy)]
enum Command {
    Forward,
    Down,
    Up,
}

impl Parseable for Command {
    fn parse(p: &mut aoc::Parser<'_>) -> Result<Self, ParseError> {
        let line = p.next()?;

        Ok(match line {
            "forward" => Self::Forward,
            "up" => Self::Up,
            "down" => Self::Down,
            other => return Err(anyhow!("unsupported: {}", other).into()),
        })
    }
}

fn main() -> Result<()> {
    let input = aoc::load("02.txt")?;

    let lines = aoc::lines::<(Command, u32)>(input)?;

    let mut a = (0, 0);

    for (command, d) in lines.iter().copied() {
        match command {
            Command::Forward => {
                a.0 += d;
            }
            Command::Down => {
                a.1 += d;
            }
            Command::Up => {
                a.1 -= d;
            }
        }
    }

    assert_eq!(a.0 * a.1, 2322630);

    let mut b = (0, 0, 0);

    for (command, d) in lines.iter().copied() {
        match command {
            Command::Forward => {
                b.0 += d;
                b.1 += b.2 * d;
            }
            Command::Down => {
                b.2 += d;
            }
            Command::Up => {
                b.2 -= d;
            }
        }
    }

    assert_eq!(b.0 * b.1, 2105273490);
    Ok(())
}
