use anyhow::Result;

fn main() -> Result<()> {
    let input = aoc::load("01.txt")?;

    let lines: Vec<u32> = aoc::lines::<u32>(input)?;

    let mut last = None;
    let mut a = 0;

    for cur in lines.iter().copied() {
        if matches!(last.replace(cur), Some(last) if cur > last) {
            a += 1;
        }
    }

    assert_eq!(a, 1709);

    let mut last = None;
    let mut b = 0;

    for window in lines.windows(3) {
        let cur = window.iter().copied().sum::<u32>();

        if matches!(last.replace(cur), Some(last) if cur > last) {
            b += 1;
        }
    }

    assert_eq!(b, 1761);
    Ok(())
}
