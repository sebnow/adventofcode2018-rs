use regex::Regex;
use std::cmp;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct Claim {
    id: u32,
    rect: Rect,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

impl Rect {
    #[inline]
    pub fn height(&self) -> u32 {
        self.bottom - self.top + 1
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.right - self.left + 1
    }

    pub fn intersection(&self, b: &Rect) -> Option<Rect> {
        let c = Rect {
            top: cmp::max(self.top, b.top),
            left: cmp::max(self.left, b.left),
            bottom: cmp::min(self.bottom, b.bottom),
            right: cmp::min(self.right, b.right),
        };

        if c.left <= c.right && c.bottom >= c.top {
            Some(c)
        } else {
            None
        }
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Claim> {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    re.captures_iter(input)
        .map(|cap| {
            let x = cap[2].parse::<u32>().unwrap();
            let y = cap[3].parse::<u32>().unwrap();

            Claim {
                id: cap[1].parse::<u32>().unwrap(),
                rect: Rect {
                    left: x,
                    top: y,
                    right: x + cap[4].parse::<u32>().unwrap() - 1,
                    bottom: y + cap[5].parse::<u32>().unwrap() - 1,
                },
            }
        }).collect()
}

fn intersections(claims: &[Claim]) -> HashSet<(u32, u32)> {
    let mut squares = HashSet::new();

    pairs(claims)
        .iter()
        .filter_map(|(a, b)| {
            if a.rect == b.rect {
                return None;
            }

            a.rect.intersection(&b.rect)
        }).for_each(|intersection| {
            for x in intersection.left..=intersection.right {
                for y in intersection.top..=intersection.bottom {
                    squares.insert((x, y));
                }
            }
        });

    squares
}

#[aoc(day3, part1)]
fn answer_1(claims: &[Claim]) -> usize {
    intersections(claims).len()
}

#[aoc(day3, part2)]
fn answer_2(claims: &[Claim]) -> u32 {
    let squares = intersections(claims);

    let ids = claims
        .iter()
        .filter(|c| {
            for x in c.rect.left..=c.rect.right {
                for y in c.rect.top..=c.rect.bottom {
                    if squares.contains(&(x, y)) {
                        return false;
                    }
                }
            }

            true
        }).map(|c| c.id)
        .collect::<Vec<u32>>();

    ids[0]
}

fn pairs<'a>(xs: &'a [Claim]) -> Vec<(&'a Claim, &'a Claim)> {
    let mut v = Vec::new();

    for a in xs.iter() {
        for b in xs.into_iter() {
            v.push((a, b))
        }
    }

    v
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn intersect() {
        let a = Rect {
            top: 3,
            left: 1,
            right: 4,
            bottom: 6,
        };
        let b = Rect {
            top: 1,
            left: 3,
            right: 6,
            bottom: 4,
        };

        assert_eq!(
            a.intersection(&b),
            Some(Rect {
                top: 3,
                left: 3,
                bottom: 4,
                right: 4,
            })
        )
    }

    #[test]
    fn generator() {
        assert_eq!(
            input_generator("#123 @ 3,2: 5x4"),
            vec!(Claim {
                id: 123,
                rect: Rect {
                    left: 3,
                    top: 2,
                    right: 7,
                    bottom: 5,
                },
            })
        );
    }

    #[test]
    fn dimensions() {
        let rect = Rect {
            left: 3,
            top: 2,
            right: 7,
            bottom: 5,
        };
        assert_eq!(rect.height(), 4);
        assert_eq!(rect.width(), 5);
    }

    #[test]
    fn examples_1() {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2\n";
        assert_eq!(answer_1(&input_generator(input)), 4);

        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 4,4: 2x2\n";
        assert_eq!(answer_1(&input_generator(input)), 6);
    }

    #[test]
    fn examples_2() {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2\n";
        assert_eq!(answer_2(&input_generator(input)), 3);
    }
}
