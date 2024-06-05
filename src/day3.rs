use std::io::{self};
use std::fs::File;
use std::str::FromStr;

struct Coords {
    dir: i32,
    steps: i32,
    is_vert: bool
}

struct Line {
    coord: i32,
    tuple: (i32, i32)
}

struct Point {
    x: i32,
    y: i32
}

impl Coords {
    pub fn new(entry: &str) -> Self {
        let d: char = entry.chars().nth(0).unwrap();
        let direction: i32 = if d == 'R' || d == 'U' { 1 } else { -1 };
        let steps: i32 = i32::from_str(&entry[1..]).unwrap();
        let is_vert: bool = d == 'U' || d == 'D';
        Coords {
            dir: direction, 
            steps: steps,
            is_vert: is_vert
        }
    }
}

impl Line {
    pub fn new(coord: i32, tuple: (i32, i32)) -> Self {
        let mut t1: i32 = tuple.0;
        let mut t2: i32 = tuple.1;
        if t1 > t2 {
            let tmp: i32 = t1;
            t1 = t2;
            t2 = tmp;
        }
        Line {
            coord: coord,
            tuple: (t1, t2)
        }
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point {
            x: x,
            y: y
        }
    }
}

fn collect_lines(wire: &Vec<Coords>) -> (Vec<Line>, Vec<Line>) {
    let mut horizontal_lines: Vec<Line> = Vec::new();
    let mut vertical_lines: Vec<Line> = Vec::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for w in wire {
        let shift = w.steps * w.dir as i32;
        if w.is_vert {
            vertical_lines.push(Line::new(x, (y, y + shift)));
            y += shift;
        }
        else {
            horizontal_lines.push(Line::new(y, (x, x + shift)));
            x += shift;
        }
    }

    (horizontal_lines, vertical_lines)
}

fn is_in_range(lower: i32, upper: i32, value: i32) -> bool {
    return value > lower && value < upper;
}

fn get_intersections(horizontal: &Vec<Line>, vertical: &Vec<Line>) -> Vec<Point> {
    let mut intersections: Vec<Point> = Vec::new();
    for h in horizontal {
        for v in vertical {
            if is_in_range(h.tuple.0, h.tuple.1, v.coord) && is_in_range(v.tuple.0, v.tuple.1, h.coord) {
                intersections.push(Point::new(h.coord, v.coord));
            }
        }
    }
    intersections
}

fn manhattan_distance(pt: &Point) -> i32 {
    pt.x.abs() + pt.y.abs()
}

pub fn solution(input: io::Lines<io::BufReader<File>>) -> () {
    let mut wires: Vec<Vec<Coords>> = Vec::new();
    for line in input.flatten() {
        let coords: Vec<Coords> = line.split(",")
                       .map(|s: &str| Coords::new(s))
                       .collect();
        wires.push(coords);
    }

    let lines1 = collect_lines(&wires[0]);
    let lines2 = collect_lines(&wires[1]);

    let mut intersections = get_intersections(&lines1.0, &lines2.1);
    intersections.extend(get_intersections(&lines2.0, &lines1.1));

    let mut min_distance = i32::MAX;
    for ip in intersections {
        let distance: i32 = manhattan_distance(&ip);
        if min_distance > distance {
            min_distance = distance;
        }
    }
    println!("Answer 1: {}", min_distance);
}

