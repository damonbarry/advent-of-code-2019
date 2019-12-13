use std::collections::BTreeSet;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Copy, Clone, Debug)]
pub struct Segment {
    pub direction: Direction,
    pub distance: i32,
}

impl Segment {
    pub fn new(description: &str) -> Segment {
        let direction = match description.chars().next().unwrap() {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => unimplemented!(),
        };

        let distance = description[1..].parse::<i32>().unwrap();

        Segment {
            direction,
            distance,
        }
    }

    pub fn to_points(&self, origin: Point) -> (Point, BTreeSet<Point>) {
        let mut points = BTreeSet::new();
        for i in 1..self.distance + 1 {
            points.insert(Segment::point_at(
                origin,
                Segment {
                    direction: self.direction,
                    distance: i,
                },
            ));
        }

        assert_eq!(self.distance as usize, points.len());
        (Segment::point_at(origin, *self), points)
    }

    fn point_at(origin: Point, segment: Segment) -> Point {
        let distance = segment.distance;
        match segment.direction {
            Direction::Right => Point::new(origin.x + distance, origin.y),
            Direction::Left => Point::new(origin.x - distance, origin.y),
            Direction::Up => Point::new(origin.x, origin.y + distance),
            Direction::Down => Point::new(origin.x, origin.y - distance),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_day3_part1() {
        let input = std::fs::read_to_string("src/day03/input.txt").unwrap();
        let paths: Vec<_> = input
            .lines()
            .map(|line| {
                let mut origin = Point::new(0, 0);
                line.split(',')
                    .map(Segment::new)
                    .map(|s| {
                        let (o, points) = s.to_points(origin);
                        origin = o;
                        points
                    })
                    .flatten()
                    .collect::<BTreeSet<Point>>()
            })
            .collect();

        assert_eq!(2, paths.len());

        let intersection: Vec<_> = paths[0].intersection(&paths[1]).cloned().collect();
        let mut manhattan_distance = i64::max_value();

        for point in intersection {
            let distance: i64 = point.x.abs() as i64 + point.y.abs() as i64;
            if distance < manhattan_distance {
                manhattan_distance = distance;
            }
        }

        assert_eq!(1431, manhattan_distance);
    }
}
