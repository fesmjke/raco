# raco - is application to visualize a TSP solving algorithms

## General

The **travelling salesman problem (TSP)** asks the following question: "Given a list of cities and the distances between each pair of cities, what is the shortest possible route that visits each city exactly once and returns to the origin city?".

## About the project

This project is primarily focused on tackling the traveling salesman problem (TSP) through the application of ant colony optimization, while also providing visualizations of the problem-solving process. 

However, it has expanded its scope to encompass a broader investigation into various algorithms and optimizations related to the TSP. 

As a result, the project has evolved into a comprehensive collection of different algorithms and optimizations, all aimed at visualizing and addressing the challenges posed by the TSP.

## What is used in this project

- Rust
- [macroquad](https://github.com/not-fl3/macroquad)

## Installation and usage

> Before usage make sure that you have installed **CMake**
 
1. Download project:

``git clone https://github.com/fesmjke/raco.git``

2. Build project:

``cargo build --release``

This will install dependencies and build macroquad, and raco itself.

3. Start application:

``cargo run``

## Todo
- Solutions:
  - Exact algorithm:
    - [x] Brute-Force
  - Approximation algorithm:
    - [x] Nearest Neighbor (NN)
    - [ ] Christofides algorithm
  - Simulation:
    - [ ] Simulated annealing
    - [ ] Ant colony optimization
  - Optimization:
    - [ ] Random swapping
    - [ ] 2-opt improvement
    - [ ] *k*-opt improvement

## Examples

#### Nearest Neighbor
TODO

#### Ant colony optimization
TODO

## Note

***Code in this project is not optimal and may be buggy***
