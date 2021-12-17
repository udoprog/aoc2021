use anyhow::{anyhow, Result};
use aoc::Parser;

#[derive(Debug, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn contains(&self, v: i64) -> bool {
        v >= self.start && v <= self.end
    }
}

fn main() -> Result<()> {
    let input = aoc::load("17.txt")?;
    let line = Parser::new(&input).line()?.into_str();

    let (xr, yr) = parse(line).ok_or_else(|| anyhow!("failed to parse input"))?;

    let part1 = find_max_height(&yr).ok_or_else(|| anyhow!("height not found"))?;
    assert_eq!(part1, 4005);

    let part2 = part2(&xr, &yr);
    assert_eq!(part2, 2953);
    Ok(())
}

fn part2(rx: &Range, ry: &Range) -> usize {
    // pre-calculate some possible x and y coordinates.
    let mut xs = Vec::new();
    let mut ys = Vec::new();

    for x in 1..=rx.end {
        let mut p_x = 0;
        let mut c_x = x;

        while c_x <= rx.end && c_x != 0 {
            p_x += c_x;
            c_x = i64::max(0, c_x - 1);

            if rx.contains(p_x) {
                xs.push(x);
                break;
            }
        }
    }

    for y in ry.start..=-ry.start {
        let mut p_y = 0;
        let mut c_y = y;

        while c_y >= ry.start {
            p_y += c_y;
            c_y -= 1;

            if ry.contains(p_y) {
                ys.push(y);
                break;
            }
        }
    }

    assert!(ry.start < ry.end);

    // let mut set = HashSet::new();
    let mut count = 0;

    for &y in &ys {
        for &x in &xs {
            let mut p_y = 0;
            let mut p_x = 0;
            let mut c_y = y;
            let mut c_x = x;

            while c_x <= rx.end && c_y >= ry.start {
                p_y += c_y;
                p_x += c_x;
                c_y -= 1;
                c_x = i64::max(0, c_x - 1);

                if rx.contains(p_x) && ry.contains(p_y) {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
}

/// Finding the necessary height is pretty easy:
///
/// Any projectile shot straight upwards once it reaches the zero point again
/// will have the negative velocity of the one it was shot upwards with (just as
/// if it was affected by gravity).
///
/// The only consideration here is that due to the integration rules 1 will be
/// added immediately *after* it reaches the zero point which would cause the
/// projectile to overshoot the area after it has reached the zero point. So we
/// subtract 1 in order to ensure that the velocity once it *passes* the zero
/// point is after integration (one is added) just enough to end up within the
/// designated area.
fn find_max_height(r: &Range) -> Option<i64> {
    let v = -r.start - 1;
    series(v)
}

/// Calculate the sum of the series of numbers from 1..=v
fn series(v: i64) -> Option<i64> {
    v.checked_mul(v.checked_add(1)?)?.checked_div(2)
}

fn parse(s: &str) -> Option<(Range, Range)> {
    let (from, to) = s.split_once(": ")?.1.split_once(", ")?;

    let from = pos(from)?;
    let to = pos(to)?;

    return Some((from, to));

    fn pos(s: &str) -> Option<Range> {
        let (start, end) = s.get(2..)?.split_once("..")?;

        Some(Range {
            start: start.parse().ok()?,
            end: end.parse().ok()?,
        })
    }
}
