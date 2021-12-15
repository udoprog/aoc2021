use anyhow::Result;
use aoc::Parser;

fn main() -> Result<()> {
    let input = aoc::load("06.txt")?;
    let line = Parser::new(&input).line()?.into_str();

    let mut ages = [0u64; 9];

    for d in line.split(',') {
        ages[str::parse::<usize>(d)?] += 1;
    }

    let result = solve(ages, 80);
    assert_eq!(result, 352872);

    let result = solve(ages, 256);
    assert_eq!(result, 1604361182149);
    Ok(())
}

fn solve<const N: usize>(mut input: [u64; N], iterations: usize) -> u64 {
    for n in 0..iterations {
        input[(n + 7) % N] += input[n % N];
    }

    input.into_iter().sum::<u64>()
}
