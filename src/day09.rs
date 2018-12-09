#[derive(Debug)]
pub struct Input {
    players: usize,
    last_marble_value: usize,
}

impl AsRef<Input> for Input {
    fn as_ref(&self) -> &Input {
        self
    }
}

pub struct Circle {
    current: usize,
    marble: i64,
    marbles: Vec<usize>,
}

impl Circle {
    fn new() -> Self {
        Circle {
            current: 0,
            marble: 0,
            marbles: vec![0],
        }
    }

    pub fn place(&mut self) -> Option<usize> {
        self.marble += 1;

        if self.marble % 23 == 0 {
            self.current = self.counter_clockwise(7);
            Some(23 + self.marbles.remove(self.current))
        } else {
            let i = self.clockwise(2);
            if i == 0 {
                self.marbles.push(self.marble as usize);
                self.current = self.marbles.len() - 1;
            } else {
                self.marbles.insert(i, self.marble as usize);
                self.current = i;
            }

            None
        }
    }

    fn clockwise(&self, n: usize) -> usize {
        cycle_index(self.marbles.len(), self.current as i64 + n as i64)
    }

    fn counter_clockwise(&self, n: usize) -> usize {
        cycle_index(self.marbles.len(), self.current as i64 - n as i64)
    }
}

fn cycle_index(len: usize, i: i64) -> usize {
    match len {
        0 => 0,
        1 => 0,
        _ => if i >= 0 {
            i as usize % len
        } else {
            len - (i.abs() as usize % len)
        },
    }
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Input {
    let mut i = input.trim().split(' ');

    Input {
        players: i.nth(0).unwrap().parse().unwrap(),
        last_marble_value: i.nth(5).unwrap().parse().unwrap(),
    }
}

#[aoc(day9, part1)]
fn answer_1(input: &Input) -> usize {
    let mut players: Vec<usize> = (0..input.players).map(|_| 0).collect();
    let mut circle = Circle::new();

    for (_, player) in (1..=input.last_marble_value).zip((0..players.len()).cycle()) {
        if let Some(value) = circle.place() {
            players[player] += value;
        }
    }

    *players.iter().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn place() {
        let mut circle = Circle::new();
        circle.place();
        assert_eq!(vec![0, 1], circle.marbles);
        assert_eq!(1, circle.current);
        circle.place();
        assert_eq!(vec![0, 2, 1], circle.marbles);
        assert_eq!(1, circle.current);
        circle.place();
        assert_eq!(vec![0, 2, 1, 3], circle.marbles);
        assert_eq!(3, circle.current);
        circle.place();
        assert_eq!(vec![0, 4, 2, 1, 3], circle.marbles);
        assert_eq!(1, circle.current);
        circle.place();
        assert_eq!(vec![0, 4, 2, 5, 1, 3], circle.marbles);
        assert_eq!(3, circle.current);
        circle.place();
        assert_eq!(vec![0, 4, 2, 5, 1, 6, 3], circle.marbles);
        assert_eq!(5, circle.current);
        circle.place();
        assert_eq!(vec![0, 4, 2, 5, 1, 6, 3, 7], circle.marbles);
        assert_eq!(7, circle.current);
        circle.place();
        assert_eq!(vec![0, 8, 4, 2, 5, 1, 6, 3, 7], circle.marbles);
        assert_eq!(1, circle.current);

        for _ in 9..23 {
            circle.place();
        }
        assert_eq!(
            vec![0, 16, 8, 17, 4, 18, 9, 19, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15],
            circle.marbles
        );
        assert_eq!(13, circle.current);
        circle.place();
        assert_eq!(
            vec![0, 16, 8, 17, 4, 18, 19, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15],
            circle.marbles
        );
        assert_eq!(6, circle.current);

        circle.place();
        assert_eq!(
            vec![0, 16, 8, 17, 4, 18, 19, 2, 24, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15],
            circle.marbles
        );
        assert_eq!(8, circle.current);

        circle.place();
        assert_eq!(
            vec![
                0, 16, 8, 17, 4, 18, 19, 2, 24, 20, 25, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7,
                15
            ],
            circle.marbles
        );
        assert_eq!(10, circle.current);
        assert_eq!(25, circle.marble);
    }

    #[test]
    fn test_cycle_index() {
        assert_eq!(0, cycle_index(0, 1));
        assert_eq!(0, cycle_index(0, 2));
        assert_eq!(0, cycle_index(0, -1));

        assert_eq!(0, cycle_index(1, 1));
        assert_eq!(0, cycle_index(1, 2));
        assert_eq!(0, cycle_index(1, -1));

        assert_eq!(1, cycle_index(2, 1));
        assert_eq!(0, cycle_index(2, 2));
        assert_eq!(1, cycle_index(2, -1));

        assert_eq!(1, cycle_index(3, 1));
        assert_eq!(1, cycle_index(3, 4));
        assert_eq!(2, cycle_index(3, -1));
    }

    #[test]
    fn examples_p1_1() {
        assert_eq!(
            32,
            answer_1(&input_generator(
                "9 players; last marble is worth 32 points"
            ))
        );
    }

    #[test]
    fn examples_p1_2() {
        assert_eq!(
            8317,
            answer_1(&input_generator(
                "10 players; last marble is worth 1618 points"
            ))
        );
    }

    #[test]
    fn examples_p1_3() {
        assert_eq!(
            146373,
            answer_1(&input_generator(
                "13 players; last marble is worth 7999 points"
            ))
        );
    }

    #[test]
    fn examples_p1_4() {
        assert_eq!(
            2764,
            answer_1(&input_generator(
                "17 players; last marble is worth 1104 points"
            ))
        );
    }

    #[test]
    fn examples_p1_5() {
        assert_eq!(
            54718,
            answer_1(&input_generator(
                "21 players; last marble is worth 6111 points"
            ))
        );
    }

    #[test]
    fn examples_p1_6() {
        assert_eq!(
            37305,
            answer_1(&input_generator(
                "30 players; last marble is worth 5807 points"
            ))
        );
    }
}
