use desdemona::strategies::*;
use desdemona::Game;

fn main() {
    let mut game = Game::new();
    Random {}.solve(&mut game);
    println!("{}", game);

    let plays: Vec<String> = game.transcript.iter().map(|p| format!("{}", p)).collect();
    println!("Transcript: {}", plays.join(","));
}
