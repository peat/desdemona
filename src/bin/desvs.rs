use clap::{App, ArgMatches};
use desdemona::strategies::*;
use desdemona::{Disc, Game};
use rayon::prelude::*;
use std::io;

const DEFAULT_GAMES: usize = 1000;

pub fn main() -> Result<(), io::Error> {
    let config = get_args();

    let dark_strategy: Box<dyn Strategy> = dark_strategy(&config)?;
    let light_strategy: Box<dyn Strategy> = light_strategy(&config)?;
    let game_count: usize = match config.value_of("games") {
        Some(input) => input.parse().unwrap_or(DEFAULT_GAMES),
        None => DEFAULT_GAMES,
    };

    println!(
        "desvs: playing dark ({}) vs light ({}) for {} games",
        dark_strategy.name(),
        light_strategy.name(),
        game_count,
    );

    let games_iter = OpponentIterator::new(dark_strategy.name(), light_strategy.name(), game_count);

    let games: Vec<_> = games_iter.into_iter().collect();

    games.into_par_iter().for_each(|(dark, light)| {
        let game = run_game(&dark, &light);

        // print out a CSV of stats
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
    });

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

fn light_strategy(config: &ArgMatches) -> Result<Box<dyn Strategy>, io::Error> {
    parse_strategy(config, "light")
}

fn dark_strategy(config: &ArgMatches) -> Result<Box<dyn Strategy>, io::Error> {
    parse_strategy(config, "dark")
}

fn parse_strategy(config: &ArgMatches, name: &str) -> Result<Box<dyn Strategy>, io::Error> {
    let strategy: Box<dyn Strategy> = match config.value_of(name) {
        None => Box::new(Minimize {}),
        Some(strategy) => match Strategies::from_name(strategy) {
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

fn run_game(dark_strategy_name: &str, light_strategy_name: &str) -> Game {
    let mut game = Game::new();

    let mut dark_strategy = Strategies::from_name(dark_strategy_name).unwrap();
    let mut light_strategy = Strategies::from_name(light_strategy_name).unwrap();

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

    game
}

struct OpponentIterator {
    dark_strategy_name: String,
    light_strategy_name: String,
    games: usize,
    current: usize,
}

impl OpponentIterator {
    pub fn new(dark_strategy: &str, light_strategy: &str, games: usize) -> Self {
        Self {
            dark_strategy_name: dark_strategy.to_ascii_lowercase(),
            light_strategy_name: light_strategy.to_ascii_lowercase(),
            games,
            current: 0,
        }
    }
}

impl Iterator for OpponentIterator {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.games {
            self.current += 1;
            Some((
                self.dark_strategy_name.clone(),
                self.light_strategy_name.clone(),
            ))
        } else {
            None
        }
    }
}
