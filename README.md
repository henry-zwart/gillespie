# Gillespie

Rust implementations of Gillespie's algorithms for event-driven infectious disease spread.

## Usage

Algorithms can be run with varying parameters and models via a provided CLI

```zsh
cargo run -- [ALGORITHM] [ALGORITHM PARAMS] [MODEL] [INITIAL POPULATION] [MODEL PARAMS]
```

For example, to run Gillespie's direct algorithm for 10e3 events, on an SIR model with β=1.0, γ=0.1, μ=0.00005,
and an initial population with S=10e6 and I=5, the corresponding command is:

```zsh
cargo run -- direct --max-iters 10000 sir 1000000 5 --beta 1.0 --gamma 0.1 --mu 0.00005
```

We recommend running in release-mode (much faster), by changing `cargo run` to `cargo run --release`.
