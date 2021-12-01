use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}
pub use Color::*;

// (1, A) / (0, 0) er nede til venstre. (8, H) / (7, 7) er oppe til højre.

// The smallest possible integer is used to store cordinates, as values can only be between 0 and 8 anyways.
#[derive(Copy, Clone, Debug)]
pub struct Pos {
    pub x: i8,
    pub y: i8,
}

impl Pos {
    // Bit magit, to encode a 2D vector into a u16, for easy interfacing with javascript.
    pub fn to_u16(&self) -> u16 {
        let x = self.x;
        let y = self.y;
        if self.is_invalid() {
            // If position is invalid, return 9 (This position does not corespond to any position on the board)
            // The point here is to make some errors easier to spot in javascript. 9 means error.
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
}

// Returns a list of places a piece can move, when at s specific position.
#[wasm_bindgen]
pub fn valid_moves(piece: Piece, pos: u16, color: Color) -> Vec<u16> {
    Pos::from_u16(pos)
        .valid_moves(piece, color)
        .iter()
        .filter(|n| {
            if n.to_u16() == u16::MAX {
                #[cfg(target_family = "wasm")]
                unsafe {
                    log("crab_engine Error: Pos to u16 conversion error");
                }
                #[cfg(not(target_family = "wasm"))]
                println!("Error: Pos to u16 conversion error")
            }
            n.to_u16() != u16::MAX
        })
        .map(|n| n.to_u16())
        .collect()
}

impl Pos {
    pub fn valid_moves(&self, piece: Piece, color: Color) -> Vec<Pos> {
        let pos = self;
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
            Piece::Queen => {
                for n in 0..8 {
                    if n != pos.x {
                        buffer.push(Pos { x: n, y: pos.y });
                    }
                    if n != pos.y {
                        buffer.push(Pos { x: pos.x, y: n });
                    }
                    if n != 0 {
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
            }

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
    }
}
