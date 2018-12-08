use std::collections::VecDeque;

#[derive(Debug)]
pub struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    pub fn sum(&self) -> usize {
        self.metadata.iter().sum()
    }

    pub fn value(&self) -> usize {
        if self.children.is_empty() {
            self.sum()
        } else {
            self.metadata
                .iter()
                .flat_map(|i| self.children.get(i - 1).map(|n| n.value()))
                .sum()
        }
    }
}

impl AsRef<Node> for Node {
    fn as_ref(&self) -> &Node {
        self
    }
}

fn parse_node(i: &mut impl Iterator<Item = usize>) -> Node {
    let n_children = i.next().expect("missing children count");
    let n_metadata = i.next().expect("missing metadata count");

    Node {
        children: (0..n_children).map(|_| parse_node(i)).collect(),
        metadata: (0..n_metadata)
            .map(|_| i.next().expect("missing metadata"))
            .collect(),
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Node {
    let mut i = input.trim().split(' ').map(|x| x.parse::<usize>().unwrap());

    parse_node(&mut i)
}

#[aoc(day8, part1)]
fn answer_1(input: &Node) -> usize {
    let mut sum: usize = 0;
    let mut q = VecDeque::new();
    q.push_back(input);

    while let Some(n) = q.pop_front() {
        q.extend(n.children.iter());
        sum += n.sum();
    }

    sum
}

#[aoc(day8, part2)]
fn answer_2(input: &Node) -> usize {
    input.value()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn examples_1() {
        assert_eq!(138, answer_1(&input_generator(TEST_INPUT)));
    }

    #[test]
    fn examples_2() {
        assert_eq!(66, answer_2(&input_generator(TEST_INPUT)));
    }
}
