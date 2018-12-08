use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<(char, char)> {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut parts = l.split(' ');

            (
                parts.nth(1).unwrap().chars().next().unwrap(),
                parts.nth(5).unwrap().chars().next().unwrap(),
            )
        }).collect()
}

fn next_steps(edges: &HashMap<char, Vec<char>>, visited: &HashSet<char>) -> Vec<char> {
    edges
        .iter()
        .filter_map(|(k, v)| {
            if !visited.contains(k) && v.iter().all(|x| visited.contains(x)) {
                return Some(*k);
            }

            None
        }).collect()
}

#[aoc(day7, part1)]
fn answer_1(input: &[(char, char)]) -> String {
    let mut s = String::new();
    let mut edges: HashMap<char, Vec<char>> = HashMap::new();
    let mut visited: HashSet<char> = HashSet::new();

    for (p, c) in input {
        edges.entry(*c).or_default().push(*p);
        edges.entry(*p).or_default();
    }

    loop {
        let mut next = next_steps(&edges, &visited);
        if next.len() == 0 {
            break;
        }

        next.sort();
        s.push(next[0]);
        visited.insert(next[0]);
    }

    s
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = "\
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.\
";

    #[test]
    fn examples_1() {
        assert_eq!(answer_1(&input_generator(TEST_INPUT)), "CABDFE");
    }
}
