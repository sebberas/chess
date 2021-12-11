use std::mem;

//using the web-assembly bindings crate to talk to javascript in rust
use wasm_bindgen::prelude::*;

//defining our chesspieces
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Pawn,
    Queen,
    King,
    Knight,
    Bishop,
    Rook,
    None,
}

//defining color, so the AI will know what it'll be able to attack, and for use in the chess game logic (chess engine)
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}
pub use Color::*;

impl Color {
    pub fn not(&self) -> Self {
        match self {
            Black => White,
            White => Black,
        }
    }
    pub fn invert(&mut self) {
        *self = self.not();
    }
}

// (1, A) / (0, 0) er nede til venstre. (8, H) / (7, 7) er oppe til højre

// The smallest possible integer is used to store cordinates, as values can only be between 0 and 8 anyways
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pos {
    pub x: i8,
    pub y: i8,
}

#[wasm_bindgen]
pub struct JsPos {
    pub x: f64,
    pub y: f64,
}

pub type Move = (Pos, Pos);

impl Pos {
    // Bit magit, to encode a 2D vector into a u16, for easy interfacing with javascript
    pub fn to_u16(&self) -> u16 {
        let x = self.x;
        let y = self.y;
        if self.is_invalid() {
            // If position is invalid, return 9 (This position does not corespond to any position on the board)
            // The point here is to make some errors easier to spot in javascript. 9 means error
            return u16::MAX;
        }
        let mut buffer = [0; 2];
        buffer[0] = x as u8;
        buffer[1] = y as u8;
        u16::from_ne_bytes(buffer)
        //self.x as u16 + self.y as u16 * 8
    }
    pub fn from_u16(pos: u16) -> Self {
        //let x = pos as i8 % 8;
        //let y = (pos as i8 - x) / 8;
        let pos = pos.to_ne_bytes();
        Self {
            x: pos[0] as i8,
            y: pos[1] as i8,
        }
    }
    pub fn is_invalid(&self) -> bool {
        self.x < 0 || self.y < 0 || self.x >= 8 || self.y >= 8
    }

    pub fn parts(&self) -> [i8; 2] {
        [self.x, self.y]
    }
}

// Returns a list of places a piece can move, when at s specific position
#[wasm_bindgen]
pub fn valid_moves(piece: Piece, pos: Pos, color: Color) -> Vec<i8> {
    pos.valid_moves(piece, color)
        .iter()
        .filter(|n| {
            if n.to_u16() == u16::MAX {
                #[cfg(target_family = "wasm")]
                unsafe {
                    use crate::log;
                    log("crab_engine Error: Pos to u16 conversion error");
                }
                #[cfg(not(target_family = "wasm"))]
                println!("Error: Pos to u16 conversion error")
            }
            n.to_u16() != u16::MAX
        })
        .map(|p| p.parts())
        .flatten()
        .collect()
}

impl Pos {
    //creating valid chessmoves for the pieces, so the AI will know what it is able to do with the different pieces
    pub fn valid_moves(&self, piece: Piece, color: Color) -> Box<[Pos]> {
        let pos = self;

        match piece {
            //writing the pawn. Hardcoding the doublemove a pawn is able to do when it hasn't moved yet, therefore this is the only piece that needs to know what color it is before moving
            Piece::Pawn => {
                let mut buffer: Box<[Pos]> = unsafe {
                    if (color == White && pos.y == 1) || (color == Black && pos.y == 6) {
                        Box::new([mem::zeroed(); 2])
                    } else {
                        Box::new([mem::zeroed(); 1])
                    }
                };

                if color == White {
                    buffer[0] = Pos {
                        x: pos.x,
                        y: pos.y + 1,
                    };
                    if pos.y == 1 {
                        buffer[1] = Pos {
                            x: pos.x,
                            y: pos.y + 2,
                        };
                    }
                } else {
                    buffer[0] = Pos {
                        x: pos.x,
                        y: pos.y - 1,
                    };
                    if pos.y == 6 {
                        buffer[1] = Pos {
                            x: pos.x,
                            y: pos.y - 2,
                        };
                    }
                }
                buffer
            }
            // writing the Queen, which is a combination of the Rook and the Bishop
            Piece::Queen => {
                let mut buffer = Box::new([unsafe { mem::zeroed() }; 7 * 2 + 7 * 4]);
                let mut xn = 0;
                let mut yn = 0;
                for n in 0..8 {
                    if n != pos.x {
                        buffer[xn] = Pos { x: n, y: pos.y };
                        xn += 1;
                    }
                    if n != pos.y {
                        buffer[yn] = Pos { x: pos.x, y: n };
                        yn += 1;
                    }

                    let n = n as usize;
                    if n != 0 {
                        buffer[(n - 1) * 4 + 7 * 2] = Pos {
                            x: pos.x - n as i8,
                            y: pos.y - n as i8,
                        };

                        buffer[(n - 1) * 4 + 1 + 7 * 2] = Pos {
                            x: pos.x + n as i8,
                            y: pos.y + n as i8,
                        };

                        buffer[(n - 1) * 4 + 2 + 7 * 2] = Pos {
                            x: pos.x + n as i8,
                            y: pos.y - n as i8,
                        };

                        buffer[(n - 1) * 4 + 3 + 7 * 2] = Pos {
                            x: pos.x - n as i8,
                            y: pos.y + n as i8,
                        };
                    }
                }
                buffer
            }
            //writing the king, who is hardcoded to only move one space at a time, in a circle around the king
            Piece::King => Box::new([
                Pos {
                    x: pos.x - 1,
                    y: pos.y,
                },
                Pos {
                    x: pos.x - 1,
                    y: pos.y + 1,
                },
                Pos {
                    x: pos.x,
                    y: pos.y + 1,
                },
                Pos {
                    x: pos.x + 1,
                    y: pos.y + 1,
                },
                Pos {
                    x: pos.x + 1,
                    y: pos.y,
                },
                Pos {
                    x: pos.x + 1,
                    y: pos.y - 1,
                },
                Pos {
                    x: pos.x,
                    y: pos.y - 1,
                },
                Pos {
                    x: pos.x - 1,
                    y: pos.y - 1,
                },
            ]),

            Piece::Rook => {
                let mut buffer = [unsafe { mem::zeroed() }; 7 * 2];

                let mut xn = 0;
                let mut yn = 0;
                for n in 0..8 {
                    if n != pos.x {
                        buffer[xn * 2] = Pos { x: n, y: pos.y };
                        xn += 1;
                    }
                    if n != pos.y {
                        buffer[yn * 2 + 1] = Pos { x: pos.x, y: n };
                        yn += 1;
                    }
                }
                buffer.into()
            }

            Piece::Bishop => {
                let mut buffer = Box::new([unsafe { mem::zeroed() }; 7 * 4]);

                for n in 1..8 {
                    buffer[(n - 1) * 4] = Pos {
                        x: pos.x - n as i8,
                        y: pos.y - n as i8,
                    };

                    buffer[(n - 1) * 4 + 1] = Pos {
                        x: pos.x + n as i8,
                        y: pos.y + n as i8,
                    };

                    buffer[(n - 1) * 4 + 2] = Pos {
                        x: pos.x + n as i8,
                        y: pos.y - n as i8,
                    };

                    buffer[(n - 1) * 4 + 3] = Pos {
                        x: pos.x - n as i8,
                        y: pos.y + n as i8,
                    };
                }

                buffer
            }

            //writing the knight, because of the piece's special way to move, it made sense to make it check for validmoves by turning the board into a matrix
            Piece::Knight => Box::new([
                Pos {
                    x: -2 + pos.x,
                    y: -1 + pos.y,
                },
                Pos {
                    x: -1 + pos.x,
                    y: -2 + pos.y,
                },
                Pos {
                    x: 2 + pos.x,
                    y: -1 + pos.y,
                },
                Pos {
                    x: 1 + pos.x,
                    y: -2 + pos.y,
                },
                Pos {
                    x: 1 + pos.x,
                    y: 2 + pos.y,
                },
                Pos {
                    x: 2 + pos.x,
                    y: 1 + pos.y,
                },
                Pos {
                    x: -1 + pos.x,
                    y: 2 + pos.y,
                },
                Pos {
                    x: -2 + pos.x,
                    y: 1 + pos.y,
                },
            ]),

            //making sure that spot on the board without a piece is read correctly
            Piece::None => Box::new([]),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Board, Color::*, Piece::*, Pos};

    //#[test]
    //fn pawn_attack() {
    //    let mut b = Board::default();
    //    b.0[1][2] = (Pawn, Black);
    //    b.0[0][2] = (Pawn, Black);
    //    let moves = b.can_move(Pawn, Pos { x: 0, y: 1 }, White);

    //    assert!(
    //        moves.iter().any(|p| *p == Pos { x: 1, y: 2 }),
    //        "{:?}",
    //        moves
    //    );

    //    assert!(
    //        !moves.iter().any(|p| *p == Pos { x: 0, y: 2 }),
    //        "{:?}",
    //        moves
    //    );

    //    assert!(
    //        !moves.iter().any(|p| *p == Pos { x: 0, y: 3 }),
    //        "{:?}",
    //        moves
    //    );
    //}
}
