use anyhow::Result;
use bittle::{BitSet, Mask};

#[derive(Debug, Clone)]
struct Board {
    rows: [BitSet<u128>; 5],
    cols: [BitSet<u128>; 5],
}

impl Board {
    /// Sum all unmarked numbers.
    fn sum(&self) -> usize {
        self.rows.iter().flat_map(|row| row.iter()).sum::<usize>()
    }

    /// Set the given number on the board and indicate if we have a win.
    fn set(&mut self, num: usize) -> bool {
        for row in &mut self.rows {
            row.clear(num);

            if *row == BitSet::<u128>::empty() {
                return true;
            }
        }

        for col in &mut self.cols {
            col.clear(num);

            if *col == BitSet::<u128>::empty() {
                return true;
            }
        }

        false
    }
}

fn main() -> Result<()> {
    let input = aoc::load("04.txt")?;
    let mut p = aoc::LineParser::new(&input);

    let line = p.line()?;

    let mut nums: Vec<usize> = Vec::new();

    for n in line.split(',') {
        nums.push(str::parse(n)?);
    }

    let mut boards = Vec::new();

    while p.next().is_some() {
        let mut rows = [BitSet::empty(); 5];
        let mut cols = [BitSet::empty(); 5];

        for row in &mut rows {
            let line = p.parse::<[usize; 5]>()?;

            for (col, d) in cols.iter_mut().zip(line) {
                row.set(d);
                col.set(d);
            }
        }

        boards.push(Board { rows, cols });
    }

    assert_eq!(part1(&nums, &boards), Some(51034));
    assert_eq!(part2(&nums, &boards), Some(5434));
    Ok(())
}

/// Find the result.
fn part1(nums: &[usize], boards: &[Board]) -> Option<usize> {
    let mut boards = boards.to_vec();

    for n in nums {
        for board in &mut boards {
            if board.set(*n) {
                return Some(board.sum() * *n);
            }
        }
    }

    None
}

/// Find the result.
fn part2(nums: &[usize], boards: &[Board]) -> Option<usize> {
    let mut active = BitSet::<u128>::empty();

    for b in 0..boards.len() {
        active.set(b);
    }

    let mut boards = boards.to_vec();
    let mut last = None;

    for n in nums {
        if active == BitSet::<u128>::empty() {
            break;
        }

        for (id, board) in active.join(boards.iter_mut().enumerate()) {
            if board.set(*n) {
                last = Some((id, *n));
                active.clear(id);
            }
        }
    }

    if let Some((last, n)) = last {
        Some(boards[last].sum() * n)
    } else {
        None
    }
}
