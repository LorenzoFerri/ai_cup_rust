use crate::point::Point;
use std::fs::read_to_string;

#[derive(Debug)]
pub struct Problem {
    name: String,
    comment: String,
    dimension: u32,
    best_known: u32,
    points: Vec<Point>,
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
                let mut points: Vec<Point> = vec![];

                for line in lines {
                    let words: Vec<_> = line.splitn(3, " ").collect();
                    match words.as_slice() {
                        ["NAME", ":", x] => name = x.parse().ok(),
                        ["COMMENT", ":", x] => comment = x.parse().ok(),
                        ["DIMENSION", ":", x] => dimension = x.parse::<u32>().unwrap(),
                        ["BEST_KNOWN", ":", x] => best_known = x.parse::<u32>().unwrap(),
                        [_, ":", _] => continue,
                        ["EOF"] => break,
                        [id, x, y] => points.push(Point::new(
                            id.parse::<u16>().unwrap(),
                            x.parse::<f32>().unwrap() as u32,
                            y.parse::<f32>().unwrap() as u32,
                        )),
                        _ => continue,
                    }
                }
                Ok(Problem {
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
}
