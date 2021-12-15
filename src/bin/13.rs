use anyhow::Result;
use aoc::Parser;
use bittle::BitSet;

#[derive(Debug, Clone, Copy)]
enum Axis {
    X,
    Y,
}

#[derive(Debug, Clone, Copy)]
struct Inst {
    axis: Axis,
    d: usize,
}

fn main() -> Result<()> {
    let input = aoc::load("13.txt")?;
    let mut p = Parser::new(&input);

    let mut page = [BitSet::<[u128; 12]>::empty(); 1536];
    let mut w = 0;
    let mut h = 0;

    while let Some((x, y)) = p.next_line().and_then(parse) {
        page[x].set(y);
        w = usize::max(w, x + 1);
        h = usize::max(h, y + 1);
    }

    let mut first = false;
    let mut part1 = 0;

    while let Some(inst) = p.next_line().and_then(parse_inst) {
        match inst.axis {
            Axis::X => {
                for x in inst.d..inst.d * 2 + 1 {
                    let to_x = inst.d - (x - inst.d);

                    for y in 0..h {
                        if page[to_x].test(y) || page[x].test(y) {
                            page[to_x].set(y);
                        }
                    }
                }

                w = inst.d;
            }
            Axis::Y => {
                for y in inst.d..inst.d * 2 + 1 {
                    let to_y = inst.d - (y - inst.d);

                    for x in 0..w {
                        if page[x].test(to_y) || page[x].test(y) {
                            page[x].set(to_y);
                        }
                    }
                }

                h = inst.d;
            }
        }

        if !first {
            first = true;

            for x in 0..w {
                part1 += page[x].iter().count();
            }
        }
    }

    assert_eq!(part1, 716);
    print_page(&page, w, h);
    Ok(())
}

fn parse(s: &str) -> Option<(usize, usize)> {
    let (x, y) = s.split_once(',')?;
    Some((str::parse(x).ok()?, str::parse(y).ok()?))
}

fn parse_inst(s: &str) -> Option<Inst> {
    let (_, rest) = s.split_once("fold along ")?;
    let (axis, d) = rest.split_once('=')?;

    let axis = match axis {
        "x" => Axis::X,
        "y" => Axis::Y,
        _ => return None,
    };

    Some(Inst {
        axis,
        d: str::parse(d).ok()?,
    })
}

fn print_page<const N: usize>(page: &[BitSet<[u128; N]>], w: usize, h: usize) {
    for y in 0..h {
        for x in 0..w {
            let c = if page[x].test(y) { '#' } else { ' ' };

            print!("{}", c);
        }

        println!();
    }
}
