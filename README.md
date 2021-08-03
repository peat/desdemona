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

## Running

`cargo run --release` will start a game for you to play.

The `--release` flag is particularly important if you run compute intensive strategies like the Monte Carlo solver.

## Binaries 

* `desdemona` (the default) starts a game of Othello.

To run the other programs, use `cargo run --bin NAME`.

* `game` prints out a complete, randomly generated game.
* `stress` runs stress tests and benchmarking (note: please use cargo's `--release` flag)
* `data` regenerates data for the static data file if needed (`src/data.rs`).

## Benchmarks

Currently plays a full random game in ~600µs, and can be parallelized to ~100µs (see `bin/stress` above). This isn't important for casual play, but it's handy for analysis!

## Copyright, etc.

Copyright 2021, Peat Bakke <peat@peat.org>.

"Othello" is a registered trademark and copyright of MegaHouse Corporation.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program.  If not, see https://www.gnu.org/licenses/.
