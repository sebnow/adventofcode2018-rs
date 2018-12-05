fn iterate(units: &str) -> String {
    (b'a'..b'z' + 1)
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .fold(units.to_owned(), |polymer, c| {
            polymer
                .replace(&format!("{}{}", c, c.to_uppercase()), "")
                .replace(&format!("{}{}", c.to_uppercase(), c), "")
        })
}

#[aoc(day5, part1)]
fn answer_1(units: &str) -> usize {
    let mut polymer = units.trim().to_owned();
    loop {
        let old = polymer.len();
        polymer = iterate(&polymer);
        if old == polymer.len() {
            return polymer.len();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_1() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!(answer_1(input), 10);
    }
    //#[test]
    //fn examples_1() {
    //    let input = "dabAcCaCBAcCcaDA";
    //    assert_eq!(answer_1(&input_generator(input)), 10);

    //    let input = "dabAcCaCBAcCcaA";
    //    assert_eq!(answer_1(&input_generator(input)), 7);

    //    let input = "AabAcCaCBAcCcaDA";
    //    assert_eq!(answer_1(&input_generator(input)), 8);

    //    let input = "aA";
    //    assert_eq!(answer_1(&input_generator(input)), 0);

    //    let input = "abBA";
    //    assert_eq!(answer_1(&input_generator(input)), 0);

    //    let input = "abAB";
    //    assert_eq!(answer_1(&input_generator(input)), 4);

    //    let input = "aabAAB";
    //    assert_eq!(answer_1(&input_generator(input)), 6);
    //}

    //    #[test]
    //    fn examples_2() {
    //        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2\n";
    //        assert_eq!(answer_2(&input_generator(input)), 3);
    //    }
}
