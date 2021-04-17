use crate::point::Point;
use std::fs::read_to_string;

#[derive(Debug)]
pub struct Problem {
    pub name: String,
    pub comment: String,
    pub dimension: u32,
    pub best_known: u32,
    pub points: Vec<Point>,
}

impl Problem {
    pub fn from_file(path: &str) -> Result<Self, &str> {
        let file = read_to_string(path);
        match file {
            Ok(contents) => {
                let lines = contents.lines();

                let mut name: Option<String> = None;
                let mut comment: Option<String> = None;
                let mut dimension: u32 = 0;
                let mut best_known: u32 = 0;
                let mut points: Vec<Point> = Vec::new();

                for line in lines {
                    let words: Vec<_> = line.splitn(3, " ").collect();
                    match words.as_slice() {
                        ["NAME", ":", x] => name = x.parse().ok(),
                        ["COMMENT", ":", x] => comment = x.parse().ok(),
                        ["DIMENSION", ":", x] => dimension = x.parse::<u32>().unwrap(),
                        ["BEST_KNOWN", ":", x] => best_known = x.parse::<u32>().unwrap(),
                        [_, ":", _] => continue,
                        [id, x, y] => points.push(Point::new(
                            id.parse::<u16>().unwrap(),
                            x.parse::<f32>().unwrap() as u32,
                            y.parse::<f32>().unwrap() as u32,
                        )),
                        _ => continue,
                    }
                }
                Ok(Self {
                    name: name.unwrap_or(String::from("No name")),
                    comment: comment.unwrap_or(String::from("No comment")),
                    dimension,
                    best_known,
                    points,
                })
            }
            Err(_) => Err("Error reading the file"),
        }
    }

    /// Get a specific point shifting by 1 since on the problem
    /// file cities are stored starting from 1
    pub fn get_point(&self, id: usize) -> &Point {
        return &self.points[id - 1];
    }
}
