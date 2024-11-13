# Turing Machine AI

This repository contains the code for an AI that aims at solving the [Turing Machine board game](https://turingmachine.info/).

## General usage

It provides a CLI that solves a game given its card ids. It works with all three games modes (normal, extreme and nightmare). The CLI's first argument is the game mode, and then a list of card ids, for example:
```
cargo run --bin main -- normal 16 31 46 48
```

A test program was also implemented to compare this solver with the AI available on [the official website](https://turingmachine.info/):
```
cargo run --release --bin test
```

## Implementation
The solver works by restraining the set of possible constraints per card, until each card has a single constraint left. At that point, the solution can be constructed by intersecting the cards' constraints.

Questions are found by maximizing the expected number of eliminated constraints. Constraints are also eliminated if no combination of constraints picked on the other cards give valid solutions to the problem. This elimination step greatly reduces the number of questions that the solver needs to ask - sometimes no questions are required to solve a game!

## Possible improvements
For now, the logic to chose the best question doesn't take into accound the fact that the answers that are collected might eliminate constraints and change the expected number of eliminations of the next question. A tree search procedure such as min-max could further improve the quality of the questions that are asked by the solver.
