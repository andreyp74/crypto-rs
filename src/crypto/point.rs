use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
    a: i32,
    b: i32
}

impl Point
{
    pub fn new(x: i32, y: i32, a: i32, b: i32) -> Self {
        if i32::pow(y, 2) != i32::pow(x, 3) + a * x + b {
            panic!("({}, {}) is not on the curve", x, y);
        }

        Point {
            x: x,
            y: y,
            a: a,
            b: b
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}
