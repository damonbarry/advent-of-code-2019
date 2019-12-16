use std::collections::BTreeSet;

#[derive(PartialEq)]
pub enum DistanceFunction {
    Manhattan,
    Steps,
}

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

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    NoIntersection,
}

fn to_points(path: impl Iterator<Item = Segment>) -> BTreeSet<Point> {
    let mut origin = Point::new(0, 0);
    path.map(|segment| {
        let (o, points) = segment.to_points(origin);
        origin = o;
        points
    })
    .flatten()
    .collect::<BTreeSet<Point>>()
}

pub fn find_nearest_intersection(
    path1: impl Iterator<Item = Segment>,
    path2: impl Iterator<Item = Segment>,
    distance_function: DistanceFunction,
) -> Result<i64, Error> {
    if distance_function != DistanceFunction::Manhattan {
        unimplemented!()
    }

    let path1 = to_points(path1);
    let path2 = to_points(path2);

    let intersection: Vec<_> = path1.intersection(&path2).cloned().collect();
    let mut manhattan_distance = i64::max_value();

    if intersection.is_empty() {
        return Err(Error {
            kind: ErrorKind::NoIntersection,
        });
    }

    for point in intersection {
        let distance: i64 = point.x.abs() as i64 + point.y.abs() as i64;
        if distance < manhattan_distance {
            manhattan_distance = distance;
        }
    }

    Ok(manhattan_distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_part1_example_1() {
        let path1 = ["R8", "U5", "L5", "D3"].iter().cloned().map(Segment::new);
        let path2 = ["U7", "R6", "D4", "L4"].iter().cloned().map(Segment::new);

        let manhattan_distance =
            find_nearest_intersection(path1, path2, DistanceFunction::Manhattan).unwrap();

        assert_eq!(6, manhattan_distance);
    }

    #[test]
    fn test_day3_part1_example_2() {
        let line1 = ["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"];
        let line2 = ["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"];

        let path1 = line1.iter().cloned().map(Segment::new);
        let path2 = line2.iter().cloned().map(Segment::new);

        let manhattan_distance =
            find_nearest_intersection(path1, path2, DistanceFunction::Manhattan).unwrap();

        assert_eq!(159, manhattan_distance);
    }

    #[test]
    fn test_day3_part1_example_3() {
        let line1 = ["R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51"];
        let line2 = ["U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"];

        let path1 = line1.iter().cloned().map(Segment::new);
        let path2 = line2.iter().cloned().map(Segment::new);

        let manhattan_distance =
            find_nearest_intersection(path1, path2, DistanceFunction::Manhattan).unwrap();

        assert_eq!(135, manhattan_distance);
    }

    #[test]
    fn solve_day3_part1() {
        let input = std::fs::read_to_string("src/day03/input.txt").unwrap();
        let lines: Vec<_> = input.lines().collect();

        let path1 = lines[0].split(',').map(Segment::new);
        let path2 = lines[1].split(',').map(Segment::new);

        let manhattan_distance =
            find_nearest_intersection(path1, path2, DistanceFunction::Manhattan).unwrap();

        assert_eq!(1431, manhattan_distance);
    }
}
