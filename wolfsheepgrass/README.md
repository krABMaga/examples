# Rust: Wolf Sheep Grass (Predator Prey)
A simple implementation of the *Wolf Sheep Predation* model, fully based on the RustAB framework. The model refers to the second variations: wolves, sheep and grass are involved.

There are currently two versions of this model:
- The simulation without the visualization framework. Outputs are the interaction between 2 agents (wolf eats sheep) and animal death.
  At each output, its step number is associated;
- The simulation with the visualization framework enabled (either natively or compiled to WebAssembly). Allows the viewer to see wolves and sheep moving around the map. Wolves try to follow sheep. Grass growth is represented by different colors. Only when grass is dark green, it can be eaten by sheep; 

![](wsg.gif)

# How to run
- To run only the simulation, run `cargo run --release`.
- To run the native visualization, run `cargo make run --profile release`.
- To serve the web visualization locally, run `cargo make serve --profile release`.
  
# References:
- http://cormas.cirad.fr/en/applica/WolfSheepPredation.htm
- http://ccl.northwestern.edu/netlogo/models/WolfSheepPredation%28DockedHybrid%29