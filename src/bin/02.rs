use std::collections::HashMap;
use std::fs::read_to_string;

// use core::f64::EPSILON; // => ~2.3E-16
const EPSILON: f64 = 1E-12;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    fn is_in_boundary(&self, line: &Line) -> bool {
        let xmin = f64::min(line.p1.x, line.p2.x);
        let xmax = f64::max(line.p1.x, line.p2.x);
        let ymin = f64::min(line.p1.y, line.p2.y);
        let ymax = f64::max(line.p1.y, line.p2.y);

        self.x >= xmin && self.x <= xmax && self.y >= ymin && self.y <= ymax
    }
}

fn ccw(p: &Point, q: &Point, r: &Point) -> i32 {
    let ccw = p.x * q.y - p.y * q.x + q.x * r.y - q.y * r.x + p.y * r.x - p.x * r.y;

    if ccw < -EPSILON {
        -1
    } else if ccw < EPSILON {
        0
    } else {
        1
    }
}

impl Point {}

pub struct Line {
    pub p1: Point,
    pub p2: Point,
    pub n: Option<Point>,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Line {
        let x = p2.x - p2.y;
        let y = p1.y - p1.x;

        let a = p1.y * p2.x - p1.x * p2.y;
        let n = if a > 0.0 {
            Point { x, y }
        } else {
            Point { x: -x, y: -y }
        };

        Line { p1, p2, n: Some(n) }
    }

    fn crosses(&self, other: &Line) -> bool {
        let a = ccw(&other.p1, &other.p2, &self.p1);
        let b = ccw(&other.p1, &other.p2, &self.p2);
        let c = ccw(&self.p1, &self.p2, &other.p1);
        let d = ccw(&self.p1, &self.p2, &other.p2);

        let h1 = a * b;
        let h2 = c * d;
        // a is on the same side as b <=> h = 1
        if h1 == 1 || h2 == 1 {
            // at least one line is completely on one side of the other line
            return false;
        }
        // a and b are on different sides <=> h = -1
        if h1 == -1 && h2 == -1 {
            // both lines are on both sides of the other line
            return true;
        }

        // from here on every possibility has *at least* one point that is 'inline'
        // if one inline point is also 'in the region' of the line it is 'inline' with,
        // it has to be touching it
        a == 0 && self.p1.is_in_boundary(other)
            || b == 0 && self.p2.is_in_boundary(other)
            || c == 0 && other.p1.is_in_boundary(self)
            || d == 0 && other.p2.is_in_boundary(self)
    }
}

fn string_to_line(string: String) -> Line {
    let numbers: Vec<f64> = string
        .split(&" ")
        .map(|word| word.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();
    Line::new(
        Point {
            x: numbers[0],
            y: numbers[1],
        },
        Point {
            x: numbers[2],
            y: numbers[3],
        },
    )
}

fn read_file_rows(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for row in read_to_string(filename).unwrap().lines() {
        result.push(row.to_string())
    }

    result
}

fn main() {
    let mut states: HashMap<String, Vec<Vec<Point>>> = HashMap::new();
    let mut cities: HashMap<String, Point> = HashMap::new();
    let mut current_id = String::new();
    let path = "../data/02/DeutschlandMitStaedten.svg";
    let data = read_file_rows(path);
    for mut line in data {
        line = line.trim().to_string();
        if line.contains("id=") {
            current_id = line.split("id=").nth(1).unwrap()
                             .split(" ").nth(0).unwrap()
                             .replace("\"", "")
                             .parse().unwrap();
            if line.contains("path") {
                states.insert(current_id.clone(), Vec::new());
            } else {
                cities.insert(current_id.clone(), Point{ x: 0.0, y: 0.0 });
            }
        }
        if line.starts_with("M") {
            states.get_mut(&current_id).unwrap().push(Vec::new());
            let coords = line[1..].split(",").collect::<Vec<&str>>();
            let point = Point {
                x: coords[0].parse().unwrap(),
                y: coords[1].parse().unwrap()
            };
            if let Some(state_vec) = states.get_mut(&current_id) {
                if let Some(last_vec) = state_vec.last_mut() {
                    last_vec.push(point);
                }
            }
        }
        if line.starts_with("l") {
            let coords = line[1..].split(",").collect::<Vec<&str>>();
            let last_point = states.get(&current_id).unwrap().last().unwrap().last().unwrap();
            let point = Point {
                x: last_point.x + coords[0].parse::<f64>().unwrap(),
                y: last_point.y + coords[1].parse::<f64>().unwrap(),
            };
            if let Some(state_vec) = states.get_mut(&current_id) {
                if let Some(last_vec) = state_vec.last_mut() {
                    last_vec.push(point);
                }
            }
        }
        if line.starts_with("L") {
            let coords = line[1..].split(",").collect::<Vec<&str>>();
            let point = Point {
                x: coords[0].parse().unwrap(),
                y: coords[0].parse().unwrap(),
            };
            if let Some(last_vec) = states.get_mut(&current_id) {
                if let Some(last_vec) = last_vec.last_mut() {
                    last_vec.push(point);
                }
            }
        }
        if line.starts_with("H") {
            let coords = line[1..].split(",").collect::<Vec<&str>>();
            let last_point = states.get(&current_id).unwrap().last().unwrap().last().unwrap();
            let point = Point {
                x: coords[0].parse().unwrap(),
                y: last_point.y,
            };
            if let Some(last_vec) = states.get_mut(&current_id) {
                if let Some(last_vec) = last_vec.last_mut() {
                    last_vec.push(point);
                }
            }
        }
        if line.starts_with("sodipodi:cx") {
            if let Some(point) = cities.get_mut(&current_id) {
                point.x = line.split("=").nth(1).unwrap().replace("\"", "").parse().unwrap();
            }
        }
        if line.starts_with("sodipodi:cy") {
            if let Some(point) = cities.get_mut(&current_id) {
                point.y = line.split("=").nth(1).unwrap().replace("\"", "").parse().unwrap();
            }
        }
    }

    for (state, vec) in &states {
        println!("{} ({:?})", state, vec.len());
    }

    for (city, point) in &cities {
        println!("{} ({:?} {:?})", city, point.x, point.y);
    }
}
