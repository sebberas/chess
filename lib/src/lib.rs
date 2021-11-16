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

// (1, a) er nede til venstre. (8, h) er oppe til hoejre.
const BOARD_SIZE: u8 = 8;

fn pos_to_u16(x: u8, y: u8) -> u16 {
    let mut buffer = [0; 2];
    buffer[0] = x;
    buffer[1] = y;
    u16::from_ne_bytes(buffer)
}

#[wasm_bindgen]
pub fn valid_moves(piece: Piece, pos: u16, color: bool) -> Vec<u16> {
    let mut not_gay_array = pos.to_ne_bytes();
    let mut buffer = vec![];

    match piece {
        Piece::Pawn => todo!(),
        Piece::Queen => todo!(),
        Piece::King => todo!(),
        Piece::Knight => todo!(),
        Piece::Bishop => todo!(),
        Piece::Rook => todo!(),
        Piece::None => todo!(),
    }

    return buffer;
}

#[wasm_bindgen]
pub fn main() {
    println!("Hello, world!");
}

#[wasm_bindgen]
pub fn positions() -> i32 {
    return 1;
}
