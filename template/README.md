# krABMaga template

A starting point to develop a simulation (and a visualization) for an agent-based model with krABMaga.
The project hierarchy is modeled as follows:
- assets: a folder to store the emoji assets used to represent agents. Other types of assets should be stored here.
- src:
    - model: Rust files related to the simulation. The implementations in this folder should be strictly related to the simulation.
    - visualization: Rust files related to the visualization.
    - main.rs: The entry point of the project. There should be two `main`s, mutually exclusive, to run the simulation with or without the attached visualization.
- index.html: The entry point for the WebAssembly based visualization. Renders a simple page with the wasm.js output embedded in it.
- Makefile.toml: Cargo-make task sets to run the visualization natively or with WebAssembly. 
- Cargo.toml: A simple Cargo.toml with krABMaga already defined as a dependency and with krABMaga features exposed as first-level features.

---

![](template.gif)

---

# How to run

- To run only the simulation, run `cargo run --release`.
- To run the native visualization, run `cargo make run --profile release`.
- To serve the web visualization locally, run `cargo make serve --profile release`.
