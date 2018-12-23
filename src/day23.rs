use failure::{format_err, Error};
use regex::Regex;

#[derive(PartialEq, Debug)]
pub struct Coord(i64, i64, i64);

#[derive(PartialEq, Debug)]
pub struct Nanobot {
    pos: Coord,
    radius: u64,
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
            pos: Coord(c[1].parse()?, c[2].parse()?, c[3].parse()?),
            radius: c[4].parse()?,
        })
    }
}

impl std::fmt::Display for Nanobot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "pos=<{},{},{}>, r={}",
            self.pos.0, self.pos.1, self.pos.2, self.radius
        )
    }
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<Nanobot> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day23, part1)]
fn answer_1(input: &[Nanobot]) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = "\
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";

    #[test]
    fn test_nanobot_parser() {
        assert_eq!(
            Nanobot {
                pos: Coord(1, 2, 3),
                radius: 4
            },
            "pos=<1,2,3>, r=4".parse::<Nanobot>().unwrap(),
        );
    }

    #[test]
    fn examples_1() {
        assert_eq!(7, answer_1(&input_generator(TEST_INPUT)));
    }
}
