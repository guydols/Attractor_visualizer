# Attractor Visualizer

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)

**Attractor Visualizer** is an open-source tool (written in Rust) for visualizing mathematical attractors. The project allows users to simulate and render the trajectories of these systems.

## Features

- **Visualization of Chaotic Attractors:** Simulate and render a variety of attractors.
- **Custom Trajectory Rendering:** Observe and analyze the behavior of mathematical systems over time.
- **Designed for Extensibility:** Easily add new types of attractors.

**Currently working attractors:**
- Lorenz attractor, https://en.wikipedia.org/wiki/Lorenz_system
- Aizawa attractor, https://www.algosome.com/articles/aizawa-attractor-chaos.html
- Chen attractor,  https://en.wikipedia.org/wiki/Multiscroll_attractor
- Rössler attractor, https://en.wikipedia.org/wiki/Rössler_attractor
- Sprott attractor, https://sprott.physics.wisc.edu/SA.HTM
- Four Wing attractor, https://www.scielo.br/j/bjp/a/PZsxkWyBdyDpS3sHTQVwKZq/?format=pdf
- Burke-Shaw attractor, https://paulbourke.net/fractals/burkeshaw/
- Nosé–Hoover attractor, https://rreusser.github.io/nose-hoover-attractor/
- Rabinovich–Fabrikant attractor, https://en.wikipedia.org/wiki/Rabinovich%E2%80%93Fabrikant_equations
- Three-Scroll Unified attractor, https://en.wikipedia.org/wiki/Multiscroll_attractor
- Yu–Wang attractor, https://paulbourke.net/fractals/yuwang/

**Currently borked attractors:**
- Thomas attractor, https://en.wikipedia.org/wiki/Thomas%27_cyclically_symmetric_attractor
- Halvorsen attractor, https://brandon.nguyen.vc/blog/attractor/halvorsen/
- Dadras attractor, https://brandon.nguyen.vc/attractors/dadras/


## Getting Started

### Binaries



### From source

Clone the repository:

```sh
git clone https://github.com/guydols/Attractor_visualizer.git
cd Attractor_visualizer
```

Build or just run the project:

```sh
cargo build --release
```

```sh
cargo run --release
```


## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).
