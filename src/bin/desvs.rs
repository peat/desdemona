use clap::{App, ArgMatches};
use desdemona::strategies::*;
use desdemona::{Disc, Game};
use std::io;

const DEFAULT_GAMES: usize = 1000;

pub fn main() -> Result<(), io::Error> {
    let config = get_args();

    let mut dark_wins: usize = 0;
    let mut dark_points: usize = 0;
    let mut light_wins: usize = 0;
    let mut light_points: usize = 0;

    let mut dark_strategy: Box<dyn Strategy> = dark_strategy(&config)?;
    let mut light_strategy: Box<dyn Strategy> = light_strategy(&config)?;
    let game_count: usize = match config.value_of("games") {
        Some(input) => input.parse().unwrap_or(DEFAULT_GAMES),
        None => DEFAULT_GAMES,
    };

    let verbose = config.occurrences_of("v") > 0;

    println!(
        "desvs: playing dark ({}) vs light ({}) for {} games",
        dark_strategy.name(),
        light_strategy.name(),
        game_count,
    );

    for _ in 0..game_count {
        let mut game = Game::new();

        while !game.is_complete {
            let strategy = match game.turn {
                Disc::Dark => &mut dark_strategy,
                Disc::Light => &mut light_strategy,
            };

            match strategy.next_play(&game) {
                Some(valid_move) => game.play_valid_move(valid_move),
                None => game.pass(),
            }
        }

        if verbose {
            println!(
                "{},{},{},{},{}",
                dark_strategy.name(),
                light_strategy.name(),
                game.dark,
                game.light,
                game.transcript
                    .iter()
                    .map(|p| format!("{}", p))
                    .collect::<Vec<String>>()
                    .join("")
            );
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
        dark_strategy.name(),
        dark_wins,
        dark_points,
        light_strategy.name(),
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
            "-v                         'Verbose mode'
            -g, --games=[COUNT]        'How many games to play (default 1,000)'
            -l, --light=<STRATEGY>       'Determine the light player's strategy: minimize, maximize, random, simple, monte'
            -d, --dark=<STRATEGY>       'Determine the dark player's strategy: minimize, maximize, random, simple, monte'"
        )
        .get_matches()
}

fn light_strategy(config: &ArgMatches) -> Result<Box<dyn Strategy>, io::Error> {
    parse_strategy(config, "light")
}

fn dark_strategy(config: &ArgMatches) -> Result<Box<dyn Strategy>, io::Error> {
    parse_strategy(config, "dark")
}

fn parse_strategy(config: &ArgMatches, name: &str) -> Result<Box<dyn Strategy>, io::Error> {
    let strategy: Box<dyn Strategy> = match config.value_of(name) {
        None => Box::new(Minimize {}),
        Some(strategy) => match Strategies::from_str(strategy) {
            Some(s) => s,
            None => {
                let error = format!(
                    "Unknown strategy {} -- try random, minimize, maximize, monte, or simple.",
                    strategy
                );
                return Err(io::Error::new(io::ErrorKind::InvalidInput, error));
            }
        },
    };

    Ok(strategy)
}
