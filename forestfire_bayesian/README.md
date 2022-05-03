# ForestFire: Bayesian Optimization

It is a stochastic model to simulate the spread of a fire through a forest. Apart the dimensions of the field, the model has only one parameter: forest density.

Each tree can have one of three states:
- `Green`: the tree is alive and can burn.
- `Burning`: the tree is burning and can burn nearby trees .
- `Burned`: After a step as `Burning`, the tree is burned and can't burn anymore.

At the start, for each cell, there is a probability of `forest_density` that a tree will be `Green`. Each tree inside the first column is set to `Burning` to start fire spreading.

In this example, bayesian optimization is used to find a configuration able to execute as many steps as possible .

---

![](ff.gif)

---

# How to run

- To run only the simulation, run `cargo run --release --features bayesian`.
  