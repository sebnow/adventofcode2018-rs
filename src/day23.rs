use coord::Coord;
use failure::{format_err, Error};
use regex::Regex;

#[derive(PartialEq, Debug)]
pub struct Nanobot {
    pos: Coord,
    radius: u64,
}

impl Nanobot {
    fn in_range(&self, c: &Coord) -> bool {
        self.pos.manhattan_distance(c) <= self.radius
    }
}

impl AsRef<Nanobot> for Nanobot {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl std::str::FromStr for Nanobot {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref re: Regex = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
        }

        let c = re.captures(&s).ok_or(format_err!("error parsing {}", s))?;

        Ok(Nanobot {
            pos: Coord::new(c[1].parse()?, c[2].parse()?, c[3].parse()?),
            radius: c[4].parse()?,
        })
    }
}

impl std::fmt::Display for Nanobot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "pos=<{},{},{}>, r={}",
            self.pos.x(),
            self.pos.y(),
            self.pos.z(),
            self.radius
        )
    }
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<Nanobot> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day23, part1)]
fn answer_1(input: &[Nanobot]) -> usize {
    let strongest = input
        .iter()
        .max_by(|&a, &b| a.radius.cmp(&b.radius))
        .unwrap();

    input.iter().filter(|n| n.in_range(&strongest.pos)).count()
}

fn find_bounds<'a, I: Iterator<Item = &'a Coord>>(coords: I) -> (Coord, Coord) {
    coords.fold((Coord::default(), Coord::default()), |(min, max), coord| {
        (min.min(&coord), max.max(&coord))
    })
}

#[aoc(day23, part2)]
fn answer_2(input: &[Nanobot]) -> u64 {
    let (min, max) = find_bounds(input.iter().map(|n| &n.pos));

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_nanobot_parser() {
        assert_eq!(
            Nanobot {
                pos: Coord::new(1, 2, 3),
                radius: 4
            },
            "pos=<1,2,3>, r=4".parse::<Nanobot>().unwrap(),
        );
    }

    #[test]
    fn examples_1() {
        assert_eq!(
            7,
            answer_1(&input_generator(
                "\
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1"
            ))
        );
    }

    #[test]
    fn examples_2() {
        assert_eq!(
            36,
            answer_2(&input_generator(
                "\
pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5"
            ))
        );
    }
}
