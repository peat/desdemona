use clap::{App, ArgMatches};
use desdemona::strategies::*;
use desdemona::{Disc, Game, Play, Position};
use std::fmt::Error;
use std::io::{self, Write};

const DEFAULT_GAMES: usize = 1000;

pub fn main() -> Result<(), io::Error> {
    let config = get_args();

    let mut dark_wins: usize = 0;
    let mut dark_points: usize = 0;
    let mut light_wins: usize = 0;
    let mut light_points: usize = 0;

    let mut dark_solver: Box<dyn Strategy> = dark_solver(&config)?;
    let mut light_solver: Box<dyn Strategy> = light_solver(&config)?;
    let game_count: usize = match config.value_of("games") {
        Some(input) => input.parse().unwrap_or(DEFAULT_GAMES),
        None => DEFAULT_GAMES,
    };

    println!(
        "desvs: playing dark ({}) vs light ({}) for {} games",
        dark_solver.name(),
        light_solver.name(),
        game_count,
    );

    for _ in 0..game_count {
        let mut game = Game::new();

        while !game.is_complete {
            let solver = match game.turn {
                Disc::Dark => &mut dark_solver,
                Disc::Light => &mut light_solver,
            };

            match solver.next_play(&game) {
                Some(valid_move) => game.play_valid_move(valid_move),
                None => game.pass(),
            }
        }

        // tally up the points
        dark_points += game.dark;
        light_points += game.light;

        // ignore draws; award the winners
        if game.dark > game.light {
            dark_wins += 1;
        }

        if game.light > game.dark {
            light_wins += 1;
        }
    }

    println!(
        "Dark ({}): {} ({}) Light ({}): {} ({})",
        dark_solver.name(),
        dark_wins,
        dark_points,
        light_solver.name(),
        light_wins,
        light_points
    );

    Ok(())
}

fn get_args() -> ArgMatches<'static> {
    App::new("desvs")
        .version("0.1")
        .author("Peat Bakke <peat@peat.org>")
        .about("Plays two strategies against each other")
        .args_from_usage(
            "-g, --games=[COUNT]        'How many games to play (default 1,000)'
            -l, --light=<STRATEGY>       'Determine the light player's strategy: minimize, maximize, random, simple, monte'
            -d, --dark=<STRATEGY>       'Determine the dark player's strategy: minimize, maximize, random, simple, monte'"
        )
        .get_matches()
}

fn light_solver(config: &ArgMatches) -> Result<Box<dyn Strategy>, io::Error> {
    parse_solver(config, "light")
}

fn dark_solver(config: &ArgMatches) -> Result<Box<dyn Strategy>, io::Error> {
    parse_solver(config, "dark")
}

fn parse_solver(config: &ArgMatches, name: &str) -> Result<Box<dyn Strategy>, io::Error> {
    let solver: Box<dyn Strategy> = match config.value_of(name) {
        None => Box::new(Minimize {}),
        Some(strategy) => match strategy {
            "random" => Box::new(Random::new()),
            "minimize" => Box::new(Minimize {}),
            "maximize" => Box::new(Maximize {}),
            "simple" => Box::new(Simple {}),
            "monte" => Box::new(Monte::new()),
            e => {
                let error = format!(
                    "Unknown strategy {} -- try random, minimize, maximize, monte, or simple.",
                    e
                );
                return Err(io::Error::new(io::ErrorKind::InvalidInput, error));
            }
        },
    };

    Ok(solver)
}
