use std::collections::VecDeque;

use anyhow::Result;
use aoc::Parser;

fn main() -> Result<()> {
    let input = aoc::load("11.txt")?;
    let mut p = Parser::new(&input);

    let mut grid: Vec<u8> = Vec::new();

    while let Some(line) = p.next_line().map(Parser::into_str) {
        for c in line.chars() {
            grid.push(match c {
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                _ => 0,
            });
        }
    }

    let mut part1 = 0;
    let mut part2 = 0;

    for step in 1.. {
        let mut bump = VecDeque::new();
        bump.extend(0..100);

        while let Some((n, v)) = bump.pop_back().and_then(|n| Some((n, grid.get_mut(n)?))) {
            if *v < 9 {
                *v += 1;
                continue;
            }

            if *v != 10 {
                *v = 10;
                bump.extend(neigh(n));
            }
        }

        let mut count = 0;

        for v in &mut grid {
            if *v == 10 {
                if step <= 100 {
                    part1 += 1;
                }

                *v = 0;
                count += 1;
            }
        }

        if count == grid.len() {
            part2 = step;
            break;
        }
    }

    assert_eq!(part1, 1700);
    assert_eq!(part2, 273);
    Ok(())
}

fn neigh(n: usize) -> impl Iterator<Item = usize> {
    let mut out = [None::<usize>; 8];

    out[0] = n.checked_sub(10);
    out[1] = n.checked_add(10);

    if n % 10 != 0 {
        out[2] = n.checked_sub(1);
        out[3] = n.checked_sub(11);
        out[4] = n.checked_add(9);
    }

    if n % 10 != 9 {
        out[5] = n.checked_sub(9);
        out[6] = n.checked_add(1);
        out[7] = n.checked_add(11);
    }

    out.into_iter().flat_map(|n| n).filter(|n| *n < 100)
}
