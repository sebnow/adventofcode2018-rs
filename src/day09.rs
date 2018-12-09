use std::collections::VecDeque;

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
    marble: usize,
    marbles: VecDeque<usize>,
}

impl Circle {
    fn new() -> Self {
        let mut marbles = VecDeque::new();
        marbles.push_back(0);

        Circle {
            marble: 0,
            marbles: marbles,
        }
    }

    pub fn place(&mut self) -> Option<usize> {
        self.marble += 1;

        if self.marble % 23 == 0 {
            for _ in 0..7 {
                let m = self.marbles.pop_back().unwrap();
                self.marbles.push_front(m);
            }
            Some(self.marble + self.marbles.pop_front().unwrap())
        } else {
            for _ in 0..2 {
                let m = self.marbles.pop_front().unwrap();
                self.marbles.push_back(m);
            }
            self.marbles.push_front(self.marble);
            None
        }
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

fn game(input: &Input, multiplier: usize) -> usize {
    let mut players: Vec<usize> = (0..input.players).map(|_| 0).collect();
    let mut circle = Circle::new();

    for (_, player) in (1..=input.last_marble_value * multiplier).zip((0..players.len()).cycle()) {
        if let Some(value) = circle.place() {
            players[player] += value;
        }
    }

    *players.iter().max().unwrap()
}

#[aoc(day9, part1)]
fn answer_1(input: &Input) -> usize {
    game(input, 1)
}

#[aoc(day9, part2)]
fn answer_2(input: &Input) -> usize {
    game(input, 100)
}

#[cfg(test)]
mod test {
    use super::*;

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
