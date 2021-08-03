use desdemona::solvers::{Random, Solver};
use desdemona::{Disc, Game};
use rand::prelude::*;
use std::io::{self, Write};
use std::time::Instant;

// grinder loops
const REPLAY_LOOPS: usize = 10_000;

fn main() -> Result<(), io::Error> {
    replay_loops()?;
    random_bench()?;
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
                good_game.valid_moves(Disc::Dark),
                good_game.valid_moves(Disc::Light)
            );

            println!("Transcript game valid moves:");
            println!(
                "Dark {:?}\nLight {:?}",
                bad_game.valid_moves(Disc::Dark),
                bad_game.valid_moves(Disc::Light)
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
            Some(vm) => game.play_valid_move(vm),
            None => game.pass(),
        }
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
    print!("Benchmarking {} random games ... ", REPLAY_LOOPS);
    io::stdout().flush()?;
    let started = Instant::now();
    Random::new().bench(REPLAY_LOOPS);
    let elapsed = started.elapsed();
    println!("{:?} per game. ✅", elapsed / (REPLAY_LOOPS as u32));
    Ok(())
}
