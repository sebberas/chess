use js_sys;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Piece {
    Pawn,
    Queen,
    King,
    Knight,
    Bishop,
    Rook,
    None,
}

const BOARD_SIZE: u8 = 8;

type Pos = js_sys::Uint8Array;

#[wasm_bindgen]
pub fn valid_moves(p: Piece, pos: Pos) -> Vec<Pos> {
    let mut not_gay_array = [0; 2];
    pos.copy_to(&mut not_gay_array);
    unimplemented!();
}

#[wasm_bindgen]
pub fn main() {
    println!("Hello, world!");
}

#[wasm_bindgen]
pub fn positions() -> i32 {
    return 1;
}
