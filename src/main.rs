mod ai;
mod board;
mod controller;
mod gamestate;
mod player;

use crate::gamestate::*;

fn main() {
    println!("Welcome to tic tac toe.");
    let mut game = GameState::new();

    game.game_loop();
}
