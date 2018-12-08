#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Point(i64, i64);

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point(x, y)
    }

    pub fn manhattan_distance(&self, b: &Self) -> u64 {
        let (x1, y1) = (self.0, self.1);
        let (x2, y2) = (b.0, b.1);

        ((x1 - x2).abs() + (y1 - y2).abs()) as u64
    }

    pub fn x(&self) -> i64 {
        self.0
    }

    pub fn y(&self) -> i64 {
        self.1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn manhattan_distance_symmetrical() {
        let centre = Point(0, 0);
        assert_eq!(
            centre.manhattan_distance(&Point(1, 1)),
            centre.manhattan_distance(&Point(-1, -1))
        );
        assert_eq!(
            centre.manhattan_distance(&Point(1, 2)),
            centre.manhattan_distance(&Point(-1, -2))
        );
    }
}
