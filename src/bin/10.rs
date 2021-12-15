use anyhow::{anyhow, bail, Result};
use aoc::Parser;

#[derive(Debug, Clone, Copy)]
enum Chunk {
    Paren,
    Bracket,
    Brace,
    Angle,
}

fn main() -> Result<()> {
    let input = aoc::load("10.txt")?;
    let mut p = Parser::new(&input);

    let mut part1 = 0;
    let mut part2 = Vec::new();

    'outer: while let Some(line) = p.next_line() {
        let mut stack = Vec::new();

        for c in line.chars() {
            let chunk = match c {
                '(' => Chunk::Paren,
                '[' => Chunk::Bracket,
                '{' => Chunk::Brace,
                '<' => Chunk::Angle,
                o => {
                    let top = stack.pop().ok_or_else(|| anyhow!("corrupted line"))?;

                    match (top, o) {
                        (Chunk::Paren, ')') => (),
                        (Chunk::Bracket, ']') => (),
                        (Chunk::Brace, '}') => (),
                        (Chunk::Angle, '>') => (),
                        (_, c) => {
                            let s = match c {
                                ')' => 3,
                                ']' => 57,
                                '}' => 1197,
                                '>' => 25137,
                                c => bail!("illegal character {:?}", c),
                            };

                            part1 += s;
                            continue 'outer;
                        }
                    }

                    continue;
                }
            };

            stack.push(chunk);
        }

        stack.reverse();

        let mut p2 = 0u64;

        for chunk in stack {
            p2 *= 5;

            p2 += match chunk {
                Chunk::Paren => 1,
                Chunk::Bracket => 2,
                Chunk::Brace => 3,
                Chunk::Angle => 4,
            };
        }

        part2.push(p2);
    }

    part2.sort();
    assert!(part2.len() % 2 == 1);
    let middle = part2.len() / 2;
    let part2 = part2[middle];

    assert_eq!(part1, 362271);
    assert_eq!(part2, 1698395182);
    Ok(())
}
