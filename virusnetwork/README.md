# Rust: Virus on a Network (a SIR model for epidemics)

With this model we want to simulate how a virus (or a warm) can be spread through a network. It's an abstraction of several dynamics:
- Network is based on a graph, where:
  - `nodes` represent computers (or any other device);
  - `edges` represent communication link into the network (edges are undirected);
- A node can be in 1 of 3 possible states:
  1. `Susceptible`: a standard node and a potential victim;
  2. `Infected`: an infected node. This kind of nodes can be unaware of the infection;
  3. `Resistant`: after virus detection, this node became resistant to this kind of infection (this state simulates an antivirus update);
- At each step, communication between all nodes is performed, and infected nodes spread virus to their neighbours (for examples, as attachment of an email);
- Periodically nodes start a scan with their antivirus to detect virus; if they detect something, a recovery will be done. If the recovery is performed, there is a possibility for that node to became `Resistant`;

The network is created using `preferential attachment` algorithm; it's provided by a krABMaga macro: given a list of nodes, it adds nodes one by one, and it creates ages based on node degree.

---

![](virus.gif)

---

# How to run
- To run only the simulation, run `cargo run --release`.
- To run the native visualization, run `cargo make run --profile release`.
- To serve the web visualization locally, run `cargo make serve --profile release`.
  
# References:
- https://ccl.northwestern.edu/netlogo/models/VirusonaNetwork