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

fn hamming(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect()
}

#[aoc(day2, part2)]
fn answer_2(input: &str) -> String {
    let words: Vec<&str> = input.lines().collect();

    for a in words.iter() {
        for b in words.iter() {
            if a == b {
                continue;
            }
            let same = hamming(a, b);
            if same.len() + 1 == a.len() {
                return same;
            }
        }
    }

    String::new()
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
    fn examples_2() {
        let input = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";
        assert_eq!(answer_2(input), "fgij");
    }
}
