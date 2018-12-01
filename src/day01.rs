use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
fn answer_1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
fn answer_2(input: &[i32]) -> i32 {
    let mut history = HashSet::new();
    let mut cur = 0;

    history.insert(cur);

    loop {
        for i in input.iter() {
            cur += i;

            if history.contains(&cur) {
                return cur;
            }

            history.insert(cur);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_1() {
        assert_eq!(3, answer_1(&vec!(1, 1, 1)));
        assert_eq!(0, answer_1(&vec!(1, 1, -2)));
        assert_eq!(-6, answer_1(&vec!(-1, -2, -3)));
    }

    #[test]
    fn examples_2() {
        assert_eq!(0, answer_2(&vec!(1, -2)));
        assert_eq!(10, answer_2(&vec!(3, 3, 4, -2, -4)));
        assert_eq!(5, answer_2(&vec!(-6, 3, 8, 5, -6)));
        assert_eq!(14, answer_2(&vec!(7, 7, -2, -7, -4)));
    }
}
