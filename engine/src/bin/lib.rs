#![feature(test)]

extern crate crab_engine;
use crab_engine::*;
extern crate test;

fn main() {
    crab_engine::main();
    //let mut game = GameState::default();
    //let action = game.best_move(crate::White, 2);
    //test::black_box(game.move_piece(action));
}
