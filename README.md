# Desdemona

Would you like to play a game? In your terminal? How about Othello? You know, the game with the light and dark discs that isn't Go?

```
  a b c d e f g h
1 · · · · · · · ·
2 · · · · · · · ·
3 · · · · · · · ·
4 · · · ● ○ · · ·
5 · · · ○ ● · · ·
6 · · · · · · · ·
7 · · · · · · · ·
8 · · · · · · · ·
```

Desdemona is an Othello sandbox, providing both a game you can play, as well as a simulation framework for developing your own strategies to play against each other.

## Installing

`cargo install desdemona` does the trick.

## Running

`desdemona` will start a game. For help, add the `--help` flag.

## Binaries

* `desdemona` (the default) starts a game of Othello.
* `desgame` prints out a complete, randomly generated game.
* `desstress` runs stress tests and benchmarking (note: please use cargo's `--release` flag)
* `desdata` regenerates data for the static data file if needed (`src/data.rs`).

## Solvers

Desdemona has a simple framework for building your own game play strategies, and includes five different "solvers" in the `src/solvers` directory:

* `Maximize` which plays the move that flips the maximum number of discs.
* `Minimize` is the opposite, playing the move that flips the least number of discs.
* `Simple` plays the first move it discovers.
* `Random` plays a random valid move.
* `Monte` runs a Monte Carlo simulation on which move is the most likely to result in a win.

To play against a particular solver, use the `-s` flag. For example, to play against the Monte solver:

`desdemona -s monte`

## Benchmarks

Currently plays a full random game in ~600µs, and can be parallelized to ~100µs (see `bin/desstress` above). This isn't important for casual play, but it's handy for analysis!

## Copyright, License

Copyright 2021, Peat Bakke <peat@peat.org>.

"Othello" is a registered trademark and copyright of MegaHouse Corporation.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program.  If not, see https://www.gnu.org/licenses/.
