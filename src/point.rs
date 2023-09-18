use std::ops::{Div, Add, AddAssign};

#[derive(Debug, Copy, Clone)]
pub struct Point {
  pub x: f64,
  pub y: f64,
}

impl Point {
  pub fn distance(&self, other: &Point) -> f64 {
    ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
  }
}

impl Div<f64> for Point {
  type Output =  Point;

  fn div(self, rhs: f64) -> Point {
    Point {
      x: self.x / rhs,
      y: self.y / rhs,
    }
  }
}

impl Add<Point> for Point {
  type Output = Point; 

  fn add(self, other: Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl AddAssign<Point> for Point {
  fn add_assign(&mut self, other: Point) {
    *self = *self + other;
  }
}

