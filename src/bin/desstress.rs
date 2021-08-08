use desdemona::strategies::{Random, Strategy};
use desdemona::{Disc, Game, Position};
use rand::prelude::*;
use rayon::prelude::*;
use std::io::{self, Write};
use std::time::Instant;

// grinder loops
const REPLAY_LOOPS: usize = 100_000;

fn main() -> Result<(), io::Error> {
    replay_loops()?;
    random_bench()?;
    parallel_bench()?;
    Ok(())
}

fn replay_loops() -> Result<(), io::Error> {
    print!("Playing and replaying {} random games ... ", REPLAY_LOOPS);
    io::stdout().flush()?;
    for _ in 0..REPLAY_LOOPS {
        if let Some((good_game, bad_game)) = divergence_grinder() {
            println!(
                "\n\nDIVERGENCE!!\n\nOriginal:\n{}\nTranscript:\n{}",
                good_game, bad_game,
            );

            println!("Original game valid moves:");
            println!(
                "Dark {:?}\nLight {:?}",
                good_game
                    .valid_moves(Disc::Dark)
                    .map(Position::new)
                    .collect::<Vec<Position>>(),
                good_game
                    .valid_moves(Disc::Light)
                    .map(Position::new)
                    .collect::<Vec<Position>>()
            );

            println!("Transcript game valid moves:");
            println!(
                "Dark {:?}\nLight {:?}",
                bad_game
                    .valid_moves(Disc::Dark)
                    .map(Position::new)
                    .collect::<Vec<Position>>(),
                bad_game
                    .valid_moves(Disc::Light)
                    .map(Position::new)
                    .collect::<Vec<Position>>()
            );

            return Ok(());
        }
    }
    println!("✅");
    Ok(())
}

fn divergence_grinder() -> Option<(Game, Game)> {
    let mut game = Game::new();
    let mut rng = thread_rng();

    // complete a random move game
    while !game.is_complete {
        match game.valid_moves(game.turn).into_iter().choose(&mut rng) {
            Some(vm) => game.play(vm),
            None => game.pass(),
        };
    }

    let tg = match Game::from_transcript(&game.transcript) {
        Some(g) => g,
        None => panic!("\n\nFAILED TO GENERATE GAME FROM TRANSCRIPT!!\n\n"),
    };

    if tg != game {
        return Some((game, tg));
    }

    None
}

fn random_bench() -> Result<(), io::Error> {
    print!(
        "Benchmarking {} random games (single thread) ... ",
        REPLAY_LOOPS
    );
    io::stdout().flush()?;
    let started = Instant::now();
    Random {}.bench(REPLAY_LOOPS);
    let elapsed = started.elapsed();
    println!("{:?} per game. ✅", elapsed / (REPLAY_LOOPS as u32));
    Ok(())
}

fn parallel_bench() -> Result<(), io::Error> {
    let divisor = 10;
    let replay_loops = REPLAY_LOOPS / divisor;
    print!(
        "Benchmarking {} random games (multithreaded) ... ",
        REPLAY_LOOPS
    );
    io::stdout().flush()?;

    let started = Instant::now();
    let _ = (0..divisor)
        .into_iter()
        .collect::<Vec<usize>>()
        .par_iter()
        .map(|_| Random {}.bench(replay_loops))
        .count();
    let elapsed = started.elapsed();

    println!("{:?} per game. ✅", elapsed / (REPLAY_LOOPS as u32));
    Ok(())
}
