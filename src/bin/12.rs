use std::collections::{HashMap, VecDeque};

use anyhow::Result;
use aoc::LineParser;

#[derive(Default)]
struct Alloc {
    strings: HashMap<String, usize>,
}

impl Alloc {
    fn alloc(&mut self, s: &str) -> usize {
        if let Some(id) = self.strings.get(s) {
            *id
        } else {
            let id = self.strings.len();
            self.strings.insert(s.to_owned(), id);
            id
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Node {
    Start,
    End,
    Name(usize, bool),
}

impl Node {
    fn from_str(name: &str, alloc: &mut Alloc) -> Node {
        match name {
            "start" => Node::Start,
            "end" => Node::End,
            name => Node::Name(alloc.alloc(name), name.chars().all(char::is_uppercase)),
        }
    }
}

impl Node {
    fn name(&self) -> Option<(usize, bool)> {
        match self {
            Node::Name(id, big) => Some((*id, *big)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Step {
    from: Node,
    to: Node,
}

fn main() -> Result<()> {
    let input = aoc::load("12.txt")?;
    let mut p = LineParser::new(&input);

    let mut edges = HashMap::<_, Vec<Node>>::new();
    let mut alloc = Default::default();

    while let Some(step) = p.next().and_then(|s| parse(s, &mut alloc)) {
        assert!(step.from != step.to);

        if step.to != Node::Start {
            edges.entry(step.from).or_default().push(step.to);
        }

        if step.from != Node::Start {
            edges.entry(step.to).or_default().push(step.from);
        }
    }

    assert!(alloc.strings.len() <= 16);
    assert_eq!(solve::<16>(&edges, false), 5874);
    assert_eq!(solve::<16>(&edges, true), 153592);
    Ok(())
}

fn solve<const N: usize>(edges: &HashMap<Node, Vec<Node>>, twice: bool) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(([false; N], Node::Start, !twice));

    let mut count = 0;

    while let Some((mut visited, cur, mut twice)) = queue.pop_front() {
        if cur == Node::End {
            count += 1;
            continue;
        }

        if let Some((id, big)) = cur.name() {
            if !big && visited[id] {
                if twice {
                    continue;
                }

                twice = true;
            }

            visited[id] = true;
        }

        for n in edges.get(&cur).into_iter().flat_map(|k| k) {
            queue.push_back((visited, *n, twice));
        }
    }

    count
}

fn parse(line: &str, alloc: &mut Alloc) -> Option<Step> {
    let (from, to) = line.split_once('-')?;
    let from = Node::from_str(from, alloc);
    let to = Node::from_str(to, alloc);
    Some(Step { from, to })
}
