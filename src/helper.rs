use std::fs::File;
use std::io::{BufRead, BufReader};
use plotters::prelude::*;
use rand::{thread_rng, Rng};

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

fn generate_colors(count_of_clusters: usize) -> Vec<RGBColor> {
  let mut colors = vec![];
  let mut rng = thread_rng();

  for _i in 0..count_of_clusters {
    colors.push(RGBColor(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)));
  }
  
  colors
}

fn draw_plot(draw_centroids: Option<bool>, centroids: Vec<Point>) -> Result<(), Box<dyn std::error::Error>>{
  let root = BitMapBackend::new("outputs/k-means.png", (800, 600))
    .into_drawing_area();

  root.fill(&WHITE)?;

  let mut chart = ChartBuilder::on(&root)
    .margin(10)
    .build_cartesian_2d(0.0..10.0, 0.0..10.0)?;

  if draw_centroids.unwrap_or(false) {
    let centroid_circles: Vec<_> = centroids
      .into_iter()
      .map(|p| Circle::new((p.x, p.y), 2, Into::<RGBColor>::into(RED).filled()))
      .collect();

    chart.draw_series(centroid_circles)?;
  }

  let colors = generate_colors(centroids.len());

//  for (index, cluster_lock) in clusters.iter().enumerate() {
//    let color = colors[index];
//    let cluster = cluster_lock.read().unwrap();
//
//    let cluster_to_draw: Vec<_> = cluster.clone()
//      .into_par_iter()
//      .map(|p| Circle::new((p.x, p.y), 1, Into::<RGBColor>::into(color).filled()))
//      .collect();
//
//    chart.draw_series(cluster_to_draw)?;
//  }

    Ok(())
}
