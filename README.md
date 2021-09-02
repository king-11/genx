# Genx

Genx provides modular building blocks to run simulations of optimization and search problems using [Genetic Algorithms](https://en.wikipedia.org/wiki/Genetic_algorithm) (GA).

The vision for genx is to be a flexible and greatly extensible framework for implementing genetic algorithm applications.

genx is written in Rust. The library's API utilizes functional programming paradigm and exposes it's API in that manner only.
## Features

- [Selection](./src/selection)
- [Mutation](./src/mutation)

## Documentation

>Work in progress

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
genex = "0.1.0"
```

If you are not using Rust 2018 edition add this to your crate root:

```rust
extern crate genx;
```

## Inspiration

I started this project mainly to learn about genetic algorithms (GAs) as part of my college curriculum where am studying severals methods for soft computing. I found myself looking for simple easy to use modular functions that I can use to learn more about each step of GA.
