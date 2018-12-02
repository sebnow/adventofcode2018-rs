use rayon::prelude::*;
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

fn hamming<'a>(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .collect()
}

fn pairs<'a>(xs: &'a [&str]) -> Vec<(&'a str, &'a str)> {
    let mut v = Vec::new();

    for a in xs.iter() {
        for b in xs.into_iter() {
            v.push((a.to_owned(), b.to_owned()))
        }
    }

    v
}

fn one_letter_off(a: &str, b: &str) -> Option<String> {
    if a == b {
        return None;
    }

    let h = hamming(a, b);
    if h.len() + 1 == a.len() {
        Some(h)
    } else {
        None
    }
}

#[aoc(day2, part2)]
fn answer_2(input: &str) -> String {
    let words: Vec<&str> = input.lines().collect();

    pairs(&words)
        .iter()
        .filter_map(|(a, b)| one_letter_off(a, b))
        .next()
        .unwrap()
}

#[aoc(day2, part2, rayon)]
fn answer_2_rayon(input: &str) -> String {
    let words: Vec<&str> = input.lines().collect();

    pairs(&words)
        .par_iter()
        .filter_map(|(a, b)| one_letter_off(a, b))
        .find_first(|_| true)
        .unwrap()
        .to_owned()
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

    #[test]
    fn examples_2_rayon() {
        let input = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";
        assert_eq!(answer_2_rayon(input), "fgij");
    }
}
