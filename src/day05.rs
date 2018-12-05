use difference::Changeset;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    kind: char,
    positive: bool,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            if self.positive {
                self.kind.to_uppercase().next().unwrap()
            } else {
                self.kind
            }
        )
    }
}

impl Unit {
    fn new(u: char) -> Self {
        let kind = u.to_lowercase().next().unwrap();

        Unit {
            kind: kind,
            positive: kind != u,
        }
    }
    pub fn reacts_with(&self, u: &Unit) -> bool {
        self.kind == u.kind && self.positive != u.positive
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Unit> {
    input.chars().map(Unit::new).collect()
}

fn find_reaction(units: &[Unit]) -> Option<usize> {
    let mut last_unit = None;

    for (i, u) in units.iter().enumerate() {
        match last_unit {
            None => {
                last_unit = Some(u);
            }
            Some(prev) => if prev.reacts_with(&u) {
                return Some(i - 1);
            } else {
                last_unit = Some(u);
            },
        }
    }

    None
}

fn unit_str(units: &[Unit]) -> String {
    units
        .iter()
        .map(|u| format!("{}", u))
        .collect::<Vec<String>>()
        .join("")
}

#[aoc(day5, part1)]
fn answer_1(units: &[Unit]) -> usize {
    let mut polymer = units.to_owned();

    loop {
        let prev_str = unit_str(&polymer);
        match find_reaction(&polymer) {
            Some(i) => {
                polymer.remove(i);
                polymer.remove(i);
            }
            None => {
                println!("{}", Changeset::new(&prev_str, &unit_str(&polymer), " "));
                return 0;
            }
        }
    }
}

#[aoc(day5, part2)]
fn answer_2(units: &[Unit]) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_1() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!(answer_1(&input_generator(input)), 10);

        let input = "dabAcCaCBAcCcaA";
        assert_eq!(answer_1(&input_generator(input)), 7);

        let input = "AabAcCaCBAcCcaDA";
        assert_eq!(answer_1(&input_generator(input)), 8);

        let input = "aA";
        assert_eq!(answer_1(&input_generator(input)), 0);

        let input = "abBA";
        assert_eq!(answer_1(&input_generator(input)), 0);

        let input = "abAB";
        assert_eq!(answer_1(&input_generator(input)), 4);

        let input = "aabAAB";
        assert_eq!(answer_1(&input_generator(input)), 6);
    }

    //    #[test]
    //    fn examples_2() {
    //        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2\n";
    //        assert_eq!(answer_2(&input_generator(input)), 3);
    //    }
}
