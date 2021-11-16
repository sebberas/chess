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

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    WHITE,
    BLACK,
}
use Color::*;

// (1, A) / (0, 0) er nede til venstre. (8, H) / (7, 7) er oppe til højre.
const BOARD_SIZE: u8 = 8;

fn pos_to_u16(x: u8, y: u8) -> u16 {
    let mut buffer = [0; 2];
    buffer[0] = x;
    buffer[1] = y;
    u16::from_ne_bytes(buffer)
}

#[wasm_bindgen]
pub fn valid_moves(piece: Piece, pos: u16, color: Color) -> Vec<u16> {
    let pos = pos.to_ne_bytes();
    let mut buffer = vec![];

    match piece {
        Piece::Pawn => {
            if color == WHITE {
                buffer.push(pos_to_u16(pos[0], pos[1] + 1));
                if pos[1] == 2 {
                    buffer.push(pos_to_u16(pos[0], pos[1] + 2));
                }
            } else {
                buffer.push(pos_to_u16(pos[0], pos[1] - 1));
                if pos[1] == 6 {
                    buffer.push(pos_to_u16(pos[0], pos[1] - 2));
                }
            }
        }
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
