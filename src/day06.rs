use point::Point;
use std::collections::HashMap;

use std::sync::atomic::{AtomicUsize, Ordering};

static MAX_PROXIMITY: AtomicUsize = AtomicUsize::new(10000);

type Grid<'a> = HashMap<Point, (&'a Point, u64)>;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(", ");
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();

            Point::new(x, y)
        }).collect()
}

fn bounds(points: &[Point]) -> (Point, Point) {
    let min = Point::new(0, 0);
    let max = points.iter().fold(min.clone(), |m, p| {
        Point::new(
            if p.x() > m.x() { p.x() } else { m.x() },
            if p.y() > m.y() { p.y() } else { m.y() },
        )
    });

    (min, max)
}

#[aoc(day6, part1)]
fn answer_1(input: &[Point]) -> usize {
    let boundary = bounds(input);
    let grid = populate_grid(input, &boundary);

    input
        .iter()
        .filter(|p| is_finite(&grid, &boundary, p))
        .map(|p| grid.iter().filter(|(_, (x, _))| p == *x).count())
        .max()
        .unwrap()
}

#[aoc(day6, part2)]
fn answer_2(input: &[Point]) -> usize {
    let boundary = bounds(input);
    let mut area: usize = 0;
    let max_proximity = MAX_PROXIMITY.load(Ordering::SeqCst);

    for x in 0..=boundary.1.x() {
        for y in 0..=boundary.1.y() {
            let sum = input
                .iter()
                .map(|i| i.manhattan_distance(&Point::new(x, y)))
                .sum::<u64>() as usize;
            if sum < max_proximity {
                area += 1
            }
        }
    }

    area
}

fn populate_grid<'a>(points: &'a [Point], boundary: &(Point, Point)) -> Grid<'a> {
    let mut grid: Grid = HashMap::with_capacity(boundary.1.x() as usize * boundary.1.y() as usize);
    for x in 0..=boundary.1.x() {
        for y in 0..=boundary.1.y() {
            let c = Point::new(x, y);
            let mut closest = points
                .iter()
                .map(|p| (p, c.manhattan_distance(p)))
                .collect::<Vec<(&Point, u64)>>();
            closest.sort_by(|(_, a), (_, b)| a.cmp(b));

            if closest[0].1 != closest[1].1 {
                grid.insert(c, closest[0]);
            }
        }
    }

    grid
}

fn is_finite(grid: &Grid, boundary: &(Point, Point), p: &Point) -> bool {
    if in_points(grid, &p, (0..=boundary.1.x()).map(|x| Point::new(x, 0))) {
        return false;
    }
    if in_points(
        &grid,
        &p,
        (0..=boundary.1.x()).map(|x| Point::new(x, boundary.1.y())),
    ) {
        return false;
    }
    if in_points(grid, &p, (0..=boundary.1.y()).map(|y| Point::new(0, y))) {
        return false;
    }
    !in_points(
        grid,
        &p,
        (0..=boundary.1.y()).map(|y| Point::new(boundary.1.x(), y)),
    )
}

fn in_points<I>(grid: &Grid, p: &Point, points: I) -> bool
where
    I: IntoIterator<Item = Point>,
{
    for x in points.into_iter() {
        if let Some(g) = grid.get(&x) {
            if g.0 == p {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = "\
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    #[test]
    fn examples_1() {
        assert_eq!(answer_1(&input_generator(TEST_INPUT)), 17);
    }

    #[test]
    fn examples_2() {
        MAX_PROXIMITY.store(32, Ordering::SeqCst);
        assert_eq!(answer_2(&input_generator(TEST_INPUT)), 16);
    }
}
