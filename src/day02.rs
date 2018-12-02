use std::collections::HashMap;

fn has_repeated(input: &str, times: usize) -> bool {
    let mut counters: HashMap<char, usize> = HashMap::new();

    for c in input.chars() {
        if let Some(x) = counters.get_mut(&c) {
            *x = *x + 1;
            continue;
        }

        counters.insert(c, 1);
    }

    counters.values().find(|&&x| x == times).is_some()
}

#[aoc(day2, part1)]
fn answer_1(input: &str) -> i32 {
    let mut twice = 0;
    let mut thrice = 0;

    for l in input.lines() {
        if has_repeated(l, 2) {
            twice += 1;
        }
        if has_repeated(l, 3) {
            thrice += 1;
        }
    }

    twice * thrice
}

#[aoc(day2, part2)]
fn answer_2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_1() {
        let input = "\
            abcdef
            bababc
            abbcde
            abcccd
            aabcdd
            abcdee
            ababab\
        ";

        assert_eq!(answer_1(input), 12);
    }

    #[test]
    fn examples_2() {}
}
