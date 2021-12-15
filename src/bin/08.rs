use std::collections::HashMap;

use anyhow::Result;
use aoc::Parser;

const EIGHT: u32 = 7;
const SEVEN: u32 = 3;
const FOUR: u32 = 4;
const ONE: u32 = 2;

#[derive(Debug)]
struct Test {
    inputs: Vec<u8>,
    outputs: Vec<u8>,
}

impl Test {
    fn find_by_count(&self, count: u32) -> u8 {
        self.find(|n| n.count_ones() == count)
    }

    fn find<F>(&self, f: F) -> u8
    where
        F: Fn(u8) -> bool,
    {
        let mut out = None;

        for n in &self.inputs {
            if f(*n) {
                assert!(out.is_none());
                out = Some(*n);
            }
        }

        out.expect("expected result")
    }
}

fn main() -> Result<()> {
    let input = aoc::load("08.txt")?;
    let mut p = Parser::new(&input);

    let mut tests = Vec::new();

    while let Some(input) = p.next_line().and_then(parse) {
        tests.push(input);
    }

    let mut p1 = 0;

    for test in &tests {
        for output in &test.outputs {
            if matches!(output.count_ones(), EIGHT | SEVEN | FOUR | ONE) {
                p1 += 1;
            }
        }
    }

    let mut p2 = 0;

    for test in &tests {
        // isolate topmost component

        let mut m = HashMap::new();

        let one = test.find_by_count(ONE);
        let four = test.find_by_count(FOUR);
        let seven = test.find_by_count(SEVEN);
        let eight = test.find_by_count(EIGHT);

        let six = test.find(|n| (n ^ one).count_ones() == 6);

        let nine = test.find(|n| n.count_ones() == 6 && (n ^ (four | seven)).count_ones() == 1);
        assert_eq!(nine.count_ones(), 6);

        let two = test.find(|n| n.count_ones() == 5 && (n ^ nine).count_ones() == 3);
        assert_eq!(two.count_ones(), 5);

        let three = test.find(|n| n.count_ones() == 5 && (n ^ two).count_ones() == 2);
        assert_eq!(three.count_ones(), 5);

        let five = test.find(|n| n.count_ones() == 5 && n != two && n != three);
        assert_eq!(five.count_ones(), 5);

        let zero = test.find(|n| n != nine && (n ^ one).count_ones() == 4);

        m.insert(zero, 0);
        m.insert(one, 1);
        m.insert(two, 2);
        m.insert(three, 3);
        m.insert(four, 4);
        m.insert(five, 5);
        m.insert(six, 6);
        m.insert(seven, 7);
        m.insert(eight, 8);
        m.insert(nine, 9);

        let mut result = 0;

        for (n, out) in test.outputs.iter().enumerate() {
            result += *m.get(out).unwrap() * 10u64.pow((test.outputs.len() - 1) as u32 - n as u32);
        }

        p2 += result;
    }

    assert_eq!(p1, 554);
    assert_eq!(p2, 990964);
    Ok(())
}

fn parse(line: &str) -> Option<Test> {
    let (first, second) = line.split_once(" | ")?;
    let inputs = first.split(' ').map(to_bits).collect::<Vec<_>>();
    let outputs = second.split(' ').map(to_bits).collect::<Vec<_>>();

    Some(Test { inputs, outputs })
}

fn to_bits(s: &str) -> u8 {
    let mut n = 0;

    for c in s.chars() {
        n |= match c {
            'a' => 1,
            'b' => 2,
            'c' => 4,
            'd' => 8,
            'e' => 16,
            'f' => 32,
            'g' => 64,
            c => panic!("unsupported {}", c),
        };
    }

    n
}
