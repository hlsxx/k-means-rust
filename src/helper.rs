use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use plotters::prelude::*;
use rand::{thread_rng, Rng};
use std::sync::{Arc, RwLock};
use crate::point::Point;

fn generate_point() -> Point {
  let mut rng = thread_rng();

  Point {
    x: rng.gen_range(0.0..10.0),
    y: rng.gen_range(0.0..10.0)
  }
}

fn generate_points(count_of_points: u16) {
  let mut file = File::create("inputs/points.txt").unwrap();

  for _i in 0..count_of_points {
    let point: Point = generate_point();
    let line = format!("{} {}\n", point.x, point.y);

    file.write_all(line.as_bytes()).unwrap();
  }
}

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

fn generate_colors(count_of_clusters: usize) -> Vec<RGBColor> {
  let mut colors = vec![];
  let mut rng = thread_rng();

  for _i in 0..count_of_clusters {
    colors.push(RGBColor(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)));
  }
  
  colors
}

pub fn draw_plot(centroids: Vec<Point>, clusters: Vec<Arc<RwLock<Vec<Point>>>>, draw_centroids: Option<bool>) -> Result<(), Box<dyn std::error::Error>>{
  let root = BitMapBackend::new("outputs/k-means.png", (800, 600))
    .into_drawing_area();

  root.fill(&WHITE)?;

  let mut chart = ChartBuilder::on(&root)
    .margin(10)
    .build_cartesian_2d(0.0..10.0, 0.0..10.0)?;

  let colors = generate_colors(centroids.len());

  if draw_centroids.unwrap_or(false) {
    let centroid_circles: Vec<_> = centroids
      .into_iter()
      .map(|p| Circle::new((p.x, p.y), 2, Into::<RGBColor>::into(RED).filled()))
      .collect();

    chart.draw_series(centroid_circles)?;
  }

  //for (index, cluster_lock) in clusters.iter().enumerate() {
  //  let color = colors[index];
  //  let cluster = cluster_lock.read().unwrap();

  //  let cluster_to_draw: Vec<_> = cluster.clone()
  //    .into_par_iter()
  //    .map(|p| Circle::new((p.x, p.y), 1, Into::<RGBColor>::into(color).filled()))
  //    .collect();

  //  chart.draw_series(cluster_to_draw)?;
  //}

  Ok(())
}
