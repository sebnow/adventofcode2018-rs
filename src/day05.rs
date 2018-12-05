fn iterate(units: &str) -> String {
    (b'a'..b'z' + 1)
        .map(|c| c as char)
        .fold(units.to_owned(), |polymer, c| {
            polymer
                .replace(&format!("{}{}", c, c.to_uppercase()), "")
                .replace(&format!("{}{}", c.to_uppercase(), c), "")
        })
}

fn polymer(units: &str) -> String {
    let mut polymer = units.trim().to_owned();
    loop {
        let old = polymer.len();
        polymer = iterate(&polymer);
        if old == polymer.len() {
            return polymer;
        }
    }
}

#[aoc(day5, part1)]
fn answer_1(units: &str) -> usize {
    polymer(units).len()
}

#[aoc(day5, part2)]
fn answer_2(units: &str) -> usize {
    (b'a'..b'z' + 1)
        .map(|c| c as char)
        .map(|c| {
            let x = units
                .to_owned()
                .replace(c, "")
                .replace(c.to_uppercase().next().unwrap(), "");
            polymer(&x).len()
        }).min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_1() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!(answer_1(input), 10);
    }

    #[test]
    fn examples_2() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!(answer_2(input), 4);
    }
}
