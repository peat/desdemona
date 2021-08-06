use clap::{App, ArgMatches};
use desdemona::strategies::*;
use desdemona::{Disc, Game, Play, Position};
use std::io::{self, Write};
use text_io::read;

#[derive(Debug)]
enum Input {
    Move(Position),
    Pass,
    Transcript,
    Quit,
}

fn main() {
    let config = get_args();
    println!("{:?}", config);

    let mut game = Game::new();

    let mut solver: Box<dyn Strategy> = match config.value_of("strategy") {
        None => Box::new(Minimize {}),
        Some(strategy) => match strategy {
            "random" => Box::new(Random::new()),
            "minimize" => Box::new(Minimize {}),
            "maximize" => Box::new(Maximize {}),
            "simple" => Box::new(Simple {}),
            "monte" => Box::new(Monte::new()),
            e => {
                println!(
                    "Unknown strategy {} -- try random, minimize, maximize, monte, or simple.",
                    e
                );
                return;
            }
        },
    };

    println!("Desdemona!\n\nComputer Strategy: {}\n\nYou are the dark discs.\nPlace a disc with a coordinate (eg: \"a2\"), or pass with \"p\".\nTo get a transcript of the game, type \"t\".\nTo quit, \"q\".\nHave fun!\n", solver.name());

    while !game.is_complete {
        println!("{}", game);

        // player is always Dark for now; Dark goes first.
        if game.turn == Disc::Dark {
            // loops until we have a valid move played, the player passes, or quits
            loop {
                let input = match prompt_for_play() {
                    None => {
                        // invalid input, try again
                        println!("What?");
                        continue;
                    }
                    Some(input) => input,
                };

                match input {
                    Input::Quit => {
                        print_transcript(&game.transcript);
                        println!("See ya!");
                        return;
                    }
                    Input::Transcript => {
                        print_transcript(&game.transcript);
                        continue;
                    }
                    Input::Pass => {
                        game.pass();
                        break;
                    }
                    Input::Move(position) => match game.validate_move(game.turn, position) {
                        Some(valid_move) => {
                            // it's a good move, do it
                            game.play_valid_move(valid_move);
                            break;
                        }
                        None => {
                            // can't move there
                            println!("Invalid move.");
                            continue;
                        }
                    },
                }
            }
        } else {
            print!("Desdemona...");
            std::io::stdout().flush().unwrap();
            // opponent (light) plays a random valid move.
            match solver.next_play(&game) {
                Some(vm) => game.play_valid_move(vm),
                None => game.pass(),
            }
            println!();
        }
    }
    print_transcript(&game.transcript);
    println!("Good game!");
}

fn prompt_for_play() -> Option<Input> {
    const X_VALUES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    const Y_VALUES: [usize; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    // show a prompt
    print!("> ");
    io::stdout().flush().unwrap();

    // read from stdin until newline
    let input: String = read!();

    // take the first character as x, convert to usize
    let raw_x = input.chars().next()?;
    let lower_x = raw_x.to_ascii_lowercase();

    // catch 'q'uit or 'p'ass or look up the X index
    let x = match lower_x {
        'p' => return Some(Input::Pass),
        'q' => return Some(Input::Quit),
        't' => return Some(Input::Transcript),
        _ => X_VALUES.iter().position(|c| c == &lower_x)?,
    };

    // second character as y, convert to usize
    let raw_y = input.chars().nth(1)?;
    let natural_y = raw_y.to_digit(10)? as usize;
    let y = Y_VALUES.iter().position(|i| i == &natural_y)?;

    Some(Input::Move(Position::from_xy(x, y)))
}

fn print_transcript(transcript: &[Play]) {
    println!(
        "Transcript: {}",
        transcript
            .iter()
            .map(|p| format!("{}", p))
            .collect::<Vec<String>>()
            .join(",")
    );
}

fn get_args() -> ArgMatches<'static> {
    App::new("desdemona")
        .version("0.1")
        .author("Peat Bakke <peat@peat.org>")
        .about("Would you like to play a game?")
        .args_from_usage(
            "-s, --strategy=[STRATEGY]       'Determine the computer's strategy: minimize, maximize, random, simple, monte'"
        )
        .get_matches()
}
