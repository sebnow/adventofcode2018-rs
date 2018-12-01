use std::collections::HashSet;

fn process(cur: i32, input: &str) -> i32 {
    let (sign, num_str) = input.split_at(1);
    let num = num_str.parse::<i32>().unwrap();

    match sign {
        "+" => cur + num,
        "-" => cur - num,
        _ => panic!("unknown sign"),
    }
}

#[aoc(day1, part1)]
fn answer_1(input: &str) -> i32 {
    input.lines().fold(0, process)
}

#[aoc(day1, part2)]
fn answer_2(input: &str) -> i32 {
    let mut history = HashSet::new();
    let mut cur = 0;

    history.insert(cur);

    loop {
        for l in input.lines() {
            cur = process(cur, l);

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
        assert_eq!(3, answer_1("+1\n+1\n+1"));
        assert_eq!(0, answer_1("+1\n+1\n-2"));
        assert_eq!(-6, answer_1("-1\n-2\n-3"));
    }

    #[test]
    fn examples_2() {
        assert_eq!(0, answer_2("+1\n-1"));
        assert_eq!(10, answer_2("+3\n+3\n+4\n-2\n-4"));
        assert_eq!(5, answer_2("-6\n+3\n+8\n+5\n-6"));
        assert_eq!(14, answer_2("+7\n+7\n-2\n-7\n-4"));
    }
}
