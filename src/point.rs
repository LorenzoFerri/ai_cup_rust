#[derive(Debug)]
pub struct Point {
    id: u16,
    x: u32,
    y: u32,
}

impl Point {
    pub fn new(id: u16, x: u32, y: u32) -> Self {
        Point { id, x, y }
    }
}
