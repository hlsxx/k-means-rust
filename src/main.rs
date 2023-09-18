mod helper;
mod point;

use helper::read_points;
use point::Point;

use std::time::Instant;
use std::sync::{Arc, RwLock};
use rand::{thread_rng, Rng};
use rayon::{current_num_threads, prelude::*};

const COUNT_OF_CLUSTERS:usize = 50;
const COUNT_OF_ITERATIONS:u8 = 100;
const MATRIX: Matrix = Matrix { x: 10.0, y: 10.0 };
const NUM_OF_CPU_CORES: &str = "4";
const DRAW_CENTRAOIDS: bool = false; 

struct Matrix {
  x: f64,
  y: f64
}

fn k_means(points: &Vec<Point>) -> (Vec<Point>, Vec<Arc<RwLock<Vec<Point>>>>) {
  let mut rng = thread_rng();

  let mut centroids: Vec<Point> = (0..COUNT_OF_CLUSTERS)
    .map(|_| Point {
      x: rng.gen_range(0.0..10.0),
      y: rng.gen_range(0.0..10.0),
    })
    .collect::<Vec<Point>>();

  let mut clusters_glob: Vec<Arc<RwLock<Vec<Point>>>> = vec![];

  for _ in 0..COUNT_OF_ITERATIONS {    
    let cluster_locks: Vec<Arc<RwLock<Vec<Point>>>> =
      centroids.iter().map(|_| Arc::new(RwLock::new(Vec::new()))).collect();

    points.par_iter().for_each(|point| {
      let mut min_distance = std::f64::INFINITY;
      let mut closest_centroid = 0;
  
      // Check all centroids and calculate which is closest
      for (i, centroid) in centroids.iter().enumerate() {
        let distance = point.distance(&centroid);

        if distance < min_distance {
          min_distance = distance;
          closest_centroid = i;
        }
      }
  
      // Push new value to the appropriate cluster
      let cluster = cluster_locks[closest_centroid].clone();
      let mut write_guard = cluster.write().unwrap();
      write_guard.push(point.clone());
    });

    clusters_glob = cluster_locks.clone();

    // Calculate centroids for each cluster by SUM(clusters) / COUNT(clusters)
    for (i, cluster_lock) in cluster_locks.iter().enumerate() {
      let cluster = cluster_lock.read().unwrap();

      let centroid = cluster.par_iter()
        .map(|&point| point)
        .reduce(|| Point { x: 0.0, y: 0.0 }, |acc, point| acc + point) / cluster.len() as f64;

      centroids[i] = centroid;
    }
  }

  (centroids, clusters_glob)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  std::env::set_var("RAYON_NUM_THREADS", NUM_OF_CPU_CORES);
  
  let points = read_points();

  let k_means_calculate_start = Instant::now();
  let (centroids, clusters) = k_means(&points);
  let k_means_calculate_end = Instant::now();


  println!("K-means time consumed: {:?}", k_means_calculate_end - k_means_calculate_start);
  println!("CPU cores used: {}", current_num_threads());

  Ok(())
}
