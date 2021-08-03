use desdemona::solvers::Random;
use desdemona::solvers::Solver;
use desdemona::Game;

fn main() {
    let mut game = Game::new();
    Random::new().solve(&mut game);
    println!("{}", game);

    let plays: Vec<String> = game.transcript.iter().map(|p| format!("{}", p)).collect();
    println!("Transcript: {}", plays.join(","));
}
