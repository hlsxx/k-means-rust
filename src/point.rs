#[derive(Debug, Copy, Clone)]
struct Point {
  x: f64,
  y: f64,
}

impl Point {
  fn distance(&self, other: &Point) -> f64 {
    // sqrt((x2 - x1)^2 + (y2 - y1)^2)
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

