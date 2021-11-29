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
    White,
    Black,
}
use Color::*;

// (1, A) / (0, 0) er nede til venstre. (8, H) / (7, 7) er oppe til højre.
const BOARD_SIZE: i8 = 8;

// The smallest possible integer is used to store cordinates, as values can only be between 0 and 8 anyways.
#[derive(Copy, Clone, Debug)]
struct Pos {
    x: i8,
    y: i8,
}

impl Pos {
    // Bit magit, to encode a 2D vector into a u16, for easy interfacing with javascript.
    fn to_u16(&self) -> u16 {
        let x = self.x;
        let y = self.y;
        if x < 0 || y < 0 || x > 8 || y > 8 {
            // If position is invalid, return 9 (This position does not corespond to any position on the board)
            // The point here is to make some errors easier to spot in javascript. 9 means error.
            return u16::MAX;
        }
        let mut buffer = [0; 2];
        buffer[0] = x as u8;
        buffer[1] = y as u8;
        u16::from_ne_bytes(buffer)
    }
    fn from_u16(pos: u16) -> Self {
        let pos = pos.to_ne_bytes();
        Self {
            x: pos[0] as i8,
            y: pos[1] as i8,
        }
    }
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Returns a list of places a piece can move, when at s specific position.
#[wasm_bindgen]
pub fn valid_moves(piece: Piece, pos: u16, color: Color) -> Vec<u16> {
    let pos = Pos::from_u16(pos);
    let mut buffer = vec![];

    match piece {
        Piece::Pawn => {
            if color == White {
                buffer.push(Pos {
                    x: pos.x,
                    y: pos.y + 1,
                });
                if pos.y == 2 {
                    buffer.push(Pos {
                        x: pos.x,
                        y: pos.y + 2,
                    });
                }
            } else {
                buffer.push(Pos {
                    x: pos.x,
                    y: pos.y - 1,
                });
                if pos.y == 6 {
                    buffer.push(Pos {
                        x: pos.x,
                        y: pos.y - 2,
                    });
                }
            }
        }
        Piece::Queen => todo!(),
        Piece::King => {
            buffer.push(Pos {
                x: pos.x - 1,
                y: pos.y,
            });
            buffer.push(Pos {
                x: pos.x - 1,
                y: pos.y + 1,
            });
            buffer.push(Pos {
                x: pos.x,
                y: pos.y + 1,
            });
            buffer.push(Pos {
                x: pos.x + 1,
                y: pos.y + 1,
            });
            buffer.push(Pos {
                x: pos.x + 1,
                y: pos.y,
            });
            buffer.push(Pos {
                x: pos.x + 1,
                y: pos.y - 1,
            });
            buffer.push(Pos {
                x: pos.x,
                y: pos.y - 1,
            });
            buffer.push(Pos {
                x: pos.x - 1,
                y: pos.y - 1,
            });
        }
        Piece::Rook => {
            for n in 0..8 {
                if n != pos.x {
                    buffer.push(Pos { x: n, y: pos.y });
                }
                if n != pos.y {
                    buffer.push(Pos { x: pos.x, y: n });
                }
            }
        }
        Piece::Bishop => {
            for n in 1..8 {
                buffer.push(Pos {
                    x: pos.x - n,
                    y: pos.y - n,
                });

                buffer.push(Pos {
                    x: pos.x + n,
                    y: pos.y + n,
                });

                buffer.push(Pos {
                    x: pos.x + n,
                    y: pos.y - n,
                });

                buffer.push(Pos {
                    x: pos.x - n,
                    y: pos.y + n,
                });
            }
        }

        Piece::Knight => {
            // Bug with knight at 2, 2 og 1, 1
            let can_move_here = [
                [0, 1, 0, 1, 0],
                [1, 0, 0, 0, 1],
                [0, 0, 0, 0, 0],
                [1, 0, 0, 0, 1],
                [0, 1, 0, 1, 0],
            ];

            for x in 0..5 {
                for y in 0..5 {
                    if can_move_here[x][y] == 1 && (x != 3 || y != 3) {
                        buffer.push(Pos {
                            x: pos.x + x as i8 - 3,
                            y: pos.y + y as i8 - 3,
                        });
                    }
                }
            }
        }
        Piece::None => {}
    }

    buffer
        .iter()
        .filter(|n| {
            if n.to_u16() == u16::MAX {
                //#[cfg(target_family = "wasm")]
                //unsafe {
                //    log("crab_engine Error: Pos to u16 conversion error");
                //}
                #[cfg(not(target_family = "wasm"))]
                println!("Error: Pos to u16 conversion error")
            }
            n.to_u16() != u16::MAX
        })
        .map(|n| n.to_u16())
        .collect()
}

#[derive(Clone, Copy)]
struct Board([[(Piece, Color); 8]; 8]);

impl Board {
    fn winner(&self) -> Option<Color> {
        if !self.0.iter().flatten().any(|n| n.1 == White) {
            return Some(Black);
        } else if !self.0.iter().flatten().any(|n| n.1 == Black) {
            return Some(White);
        }
        None
    }
}

fn can_move(piece: Piece, pos: Pos, color: Color, board: Board) -> Vec<Pos> {
    unimplemented!();
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// Main function for debugging
#[wasm_bindgen]
pub fn main() {
    let mut board = ['#'; 8 * 8];

    let p = Piece::Knight;
    for pos in valid_moves(p, Pos { x: 3, y: 3 }.to_u16(), White) {
        let pos = pos.to_ne_bytes();
        board[(pos[0] + pos[1] * 8).min(63) as usize] = '.';
    }

    for x in 0..8 {
        for y in 0..8 {
            print!("{} ", board[(7 - x) + y * 8]);
        }
        println!();
    }
}
