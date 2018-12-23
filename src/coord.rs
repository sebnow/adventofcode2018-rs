use std::cmp::{max, min};

#[derive(Debug, Eq, PartialEq, Clone, Hash, Default)]
pub struct Coord(i64, i64, i64);

impl Coord {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Coord(x, y, z)
    }

    pub fn manhattan_distance(&self, b: &Self) -> u64 {
        ((self.x() - b.x()).abs() + (self.y() - b.y()).abs() + (self.z() - b.z()).abs()) as u64
    }

    pub fn x(&self) -> i64 {
        self.0
    }

    pub fn y(&self) -> i64 {
        self.1
    }

    pub fn z(&self) -> i64 {
        self.2
    }

    pub fn min(&self, b: &Self) -> Self {
        Coord::new(
            min(self.x(), b.x()),
            min(self.y(), b.y()),
            min(self.z(), b.z()),
        )
    }

    pub fn max(&self, b: &Self) -> Self {
        Coord::new(
            max(self.x(), b.x()),
            max(self.y(), b.y()),
            max(self.z(), b.z()),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn manhattan_distance_symmetrical() {
        let centre = Coord(0, 0, 0);
        assert_eq!(
            centre.manhattan_distance(&Coord(1, 1, 1)),
            centre.manhattan_distance(&Coord(-1, -1, -1))
        );
        assert_eq!(
            centre.manhattan_distance(&Coord(1, 2, 3)),
            centre.manhattan_distance(&Coord(-1, -2, 3))
        );
    }
}
