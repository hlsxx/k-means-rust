use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::point::Point;

pub fn read_points() -> Vec<Point> {
  let file = File::open("inputs/points.txt").unwrap();
  let reader = BufReader::new(file);
  let mut points = Vec::new();

  for line in reader.lines() {
    let line = line.unwrap();
    let coords = line
      .split_whitespace()
      .map(|coord| coord.parse::<f64>().expect("Not a number"))
      .collect::<Vec<f64>>();
    
    let point = Point {
      x: coords[0], 
      y: coords[1]
    };

    points.push(point);
  }

  points
}
