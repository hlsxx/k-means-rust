# ✨ k-means-rust
The aim of this project is to implement the k-means algorithm using Rust-lang. The source code includes a parallel implementation in [Rayon](https://github.com/rayon-rs/rayon).

Here are some key characteristics of the K-means algorithm:
1. Initialization: The algorithm starts by randomly selecting K cluster centroids from the dataset.
2. Assignment: Each data point is then assigned to the nearest centroid based on the Euclidean distance metric.
3. Update: The centroids of each cluster are updated by taking the mean of all data points assigned to that cluster.
4. Repeat: Steps 2 and 3 are repeated until convergence, that is, until the assignment of data points to clusters no longer changes.
5. Optimal K: The choice of K, the number of clusters, can significantly impact the clustering results, and it is often determined using heuristics or optimization techniques.

## 🚀 Generate points
If you want to create more or fewer points, you can use the "points_generator.rs" file located in the "bin" folder. Running the command below will generate points and store them in a "points.txt" file within the "inputs" folder.
```sh
cargo run --bin points_generator
```
## ✨  Usage
The "examples" folder contains multiple implementations of the k-means algorithm, each of which differs from the others in some way.

```sh
cargon run --example parallel-iterations-2
```
## 🏁 Result
The program will generate a plot and store it in the "outputs" folder.

![Example plot](https://github.com/Holes70/k-means-rust/blob/master/outputs/k-means.png)
