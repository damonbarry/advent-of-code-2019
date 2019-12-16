use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt;
use std::hash::{Hash, Hasher};

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
    pub length: i32,
}

impl Segment {
    pub fn new(description: &str) -> Self {
        let direction = match description.chars().next().unwrap() {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => unimplemented!(),
        };

        let length = description[1..].parse::<i32>().unwrap();

        Segment { direction, length }
    }

    pub fn to_points(self, origin: Point) -> (Point, BTreeSet<Point>) {
        let mut points = BTreeSet::new();
        for i in 1..=self.length {
            points.insert(Segment::point_at(
                origin,
                Segment {
                    direction: self.direction,
                    length: i,
                },
            ));
        }

        assert_eq!(self.length as usize, points.len());
        (Segment::point_at(origin, self), points)
    }

    fn point_at(origin: Point, segment: Segment) -> Point {
        let length = segment.length;
        let distance = origin.distance + length as i64;
        match segment.direction {
            Direction::Right => {
                Point::new(origin.x.checked_add(length).unwrap(), origin.y, distance)
            }
            Direction::Left => {
                Point::new(origin.x.checked_sub(length).unwrap(), origin.y, distance)
            }
            Direction::Up => Point::new(origin.x, origin.y.checked_add(length).unwrap(), distance),
            Direction::Down => {
                Point::new(origin.x, origin.y.checked_sub(length).unwrap(), distance)
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub distance: i64,
}

impl Point {
    pub fn new(x: i32, y: i32, distance: i64) -> Self {
        Point { x, y, distance }
    }

    pub fn manhattan_distance(&self) -> i64 {
        self.x.abs() as i64 + self.y.abs() as i64
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Point {{ x: {}, y: {}, distance: {}, manhattan_distance {} }}",
            self.x,
            self.y,
            self.distance,
            self.manhattan_distance()
        )
    }
}

// implement equality, hashing, ordering in terms of x and y only, not distance
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool { self.x == other.x && self.y == other.y }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.x.hash(h);
        self.y.hash(h);
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(&other)) }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering { self.x.cmp(&other.x).then(self.y.cmp(&other.y)) }
}

#[derive(Debug, Copy, Clone)]
pub struct Intersection {
    pub point1: Point,
    pub point2: Point,
}

impl Intersection {
    pub fn new(point1: Point, point2: Point) -> Self {
        Intersection { point1, point2 }
    }

    pub fn manhattan_distance(self) -> i64 {
        self.point1.manhattan_distance()
    }

    pub fn combined_steps_distance(self) -> i64 {
        self.point1.distance + self.point2.distance
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
    let mut origin = Point::new(0, 0, 0);
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
) -> Result<Intersection, Error> {
    let path1 = to_points(path1);
    let path2 = to_points(path2);

    let intersections: Vec<_> = path1.intersection(&path2).cloned().collect();

    let mut result: Option<Intersection> = None;

    for point in intersections {
        match distance_function {
            DistanceFunction::Manhattan => {
                if result.is_none() || point.manhattan_distance() < result.unwrap().manhattan_distance() {
                    result = Some(Intersection::new(*path1.get(&point).unwrap(), *path2.get(&point).unwrap()));
                }
            }
            DistanceFunction::Steps => {
                let intersection = Intersection::new(*path1.get(&point).unwrap(), *path2.get(&point).unwrap());
                if result.is_none() || intersection.combined_steps_distance() < (result.unwrap().combined_steps_distance()) {
                    result = Some(intersection);
                }
            }
        }
    }

    result.ok_or(Error {
        kind: ErrorKind::NoIntersection,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_part1_example_1() {
        let path1 = ["R8", "U5", "L5", "D3"].iter().cloned().map(Segment::new);
        let path2 = ["U7", "R6", "D4", "L4"].iter().cloned().map(Segment::new);

        let intersection = find_nearest_intersection(path1, path2, DistanceFunction::Manhattan).unwrap();

        assert_eq!(6, intersection.manhattan_distance());
    }

    #[test]
    fn test_day3_part1_example_2() {
        let line1 = ["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"];
        let line2 = ["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"];

        let path1 = line1.iter().cloned().map(Segment::new);
        let path2 = line2.iter().cloned().map(Segment::new);

        let intersection = find_nearest_intersection(path1, path2, DistanceFunction::Manhattan).unwrap();

        assert_eq!(159, intersection.manhattan_distance());
    }

    #[test]
    fn test_day3_part1_example_3() {
        let line1 = [
            "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
        ];
        let line2 = [
            "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
        ];

        let path1 = line1.iter().cloned().map(Segment::new);
        let path2 = line2.iter().cloned().map(Segment::new);

        let intersection = find_nearest_intersection(path1, path2, DistanceFunction::Manhattan).unwrap();

        assert_eq!(135, intersection.manhattan_distance());
    }

    #[test]
    fn solve_day3_part1() {
        let input = std::fs::read_to_string("src/day03/input.txt").unwrap();
        let lines: Vec<_> = input.lines().collect();

        let path1 = lines[0].split(',').map(Segment::new);
        let path2 = lines[1].split(',').map(Segment::new);

        let intersection = find_nearest_intersection(path1, path2, DistanceFunction::Manhattan).unwrap();

        assert_eq!(1431, intersection.manhattan_distance());
    }

    #[test]
    fn test_day3_part2_example_1() {
        let path1 = ["R8", "U5", "L5", "D3"].iter().cloned().map(Segment::new);
        let path2 = ["U7", "R6", "D4", "L4"].iter().cloned().map(Segment::new);

        let intersection = find_nearest_intersection(path1, path2, DistanceFunction::Steps).unwrap();

        assert_eq!(30, intersection.combined_steps_distance());
    }

    #[test]
    fn test_day3_part2_example_2() {
        let line1 = ["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"];
        let line2 = ["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"];

        let path1 = line1.iter().cloned().map(Segment::new);
        let path2 = line2.iter().cloned().map(Segment::new);

        let intersection = find_nearest_intersection(path1, path2, DistanceFunction::Steps).unwrap();

        assert_eq!(610, intersection.combined_steps_distance());
    }

    #[test]
    fn test_day3_part2_example_3() {
        let line1 = [
            "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
        ];
        let line2 = [
            "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
        ];

        let path1 = line1.iter().cloned().map(Segment::new);
        let path2 = line2.iter().cloned().map(Segment::new);

        let intersection = find_nearest_intersection(path1, path2, DistanceFunction::Steps).unwrap();

        assert_eq!(410, intersection.combined_steps_distance());
    }

    #[test]
    fn solve_day3_part2() {
        let input = std::fs::read_to_string("src/day03/input.txt").unwrap();
        let lines: Vec<_> = input.lines().collect();

        let path1 = lines[0].split(',').map(Segment::new);
        let path2 = lines[1].split(',').map(Segment::new);

        let intersection = find_nearest_intersection(path1, path2, DistanceFunction::Steps).unwrap();

        assert_eq!(48012, intersection.combined_steps_distance());
    }
}
