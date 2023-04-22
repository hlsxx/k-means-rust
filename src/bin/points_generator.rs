use std::fs::File;
use std::io::Write;
use rand::{thread_rng, Rng};

const COUNT_OF_POINTS: u32 = 100000;

struct Point {
  x: f64,
  y: f64
}

fn generate_point() -> Point {
  let mut rng = thread_rng();

  Point {
    x: rng.gen_range(0.0..10.0),
    y: rng.gen_range(0.0..10.0)
  }
}

fn main() {
  let mut file = File::create("inputs/points.txt").unwrap();

  for _i in 0..COUNT_OF_POINTS {
    let point: Point = generate_point();
    let line = format!("{} {}\n", point.x, point.y);

    file.write_all(line.as_bytes()).unwrap();
  }
}