use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Work(usize, char);

impl Work {
    pub fn is_finished(&self, time: usize) -> bool {
        self.0 + time_required(self.1) < time
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Box<HashMap<char, Vec<char>>> {
    let i = input.trim().lines().map(|l| {
        let mut parts = l.split(' ');

        (
            parts.nth(1).unwrap().chars().next().unwrap(),
            parts.nth(5).unwrap().chars().next().unwrap(),
        )
    });

    let mut edges: HashMap<char, Vec<char>> = HashMap::new();
    for (p, c) in i {
        edges.entry(c).or_default().push(p);
        edges.entry(p).or_default();
    }

    Box::new(edges)
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
fn answer_1(edges: &HashMap<char, Vec<char>>) -> String {
    let mut s = String::new();
    let mut visited: HashSet<char> = HashSet::new();

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

fn time_required(step: char) -> usize {
    let step_delay = 60;
    step_delay + (step as u8 - 'A' as u8) as usize
}

#[aoc(day7, part2)]
fn answer_2(edges: &HashMap<char, Vec<char>>) -> usize {
    let mut total_time = 0;
    let n_workers = 5;
    let mut workers: Vec<Work> = Vec::with_capacity(n_workers);

    let mut visited: HashSet<char> = HashSet::new();

    loop {
        let mut new_workers = Vec::with_capacity(n_workers);
        while let Some(w) = workers.pop() {
            if w.is_finished(total_time) {
                visited.insert(w.1);
            } else {
                new_workers.push(w);
            }
        }

        let mut next = next_steps(&edges, &visited);
        if next.len() == 0 {
            break;
        }

        next.sort();
        for step in next {
            if new_workers.len() >= n_workers {
                break;
            }

            if new_workers.iter().find(|w| w.1 == step).is_none() {
                new_workers.push(Work(total_time, step));
            }
        }

        workers = new_workers;
        total_time += 1;
        assert!(workers.len() <= n_workers);
    }

    total_time
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
    #[test]
    fn examples_2() {
        assert_eq!(answer_2(&input_generator(TEST_INPUT)), 253);
    }

}
