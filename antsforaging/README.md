# Rust AntsForaging
A simple implementation of the Ants Foraging simulation, fully based on the RustAB framework.
There are currently two versions:
- The simulation without the visualization framework. Outputs the number of steps
required for the ants to find the food and return to their nest for the first time;
- The simulation with the visualization framework enabled (either natively or compiled to WebAssembly). Allows the viewer to see the random
paths taken by the ants while they look for food and avoid obstacles, and the pheromone distribution around the grid hotspots
  (nest and food sites).

![](ant1.gif)

![](ant2.gif)

# How to run
- To run only the simulation, run `cargo run --release`.
- To run the native visualization, run `cargo make run --release`.
- To serve the web visualization locally, run `cargo make serve --release`.
  
# References:
- https://github.com/eclab/mason/tree/master/mason/src/main/java/sim/app/antsforage