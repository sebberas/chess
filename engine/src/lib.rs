#![feature(test)]

use std::io::{stdout, Write};

use anyhow::*;
//pub use rayon::prelude::*;
use wasm_bindgen::prelude::*;

//#[cfg(target_family = "wasm")]
//pub use wasm_bindgen_rayon::init_thread_pool;

mod deepblue;
mod pieces;
pub use deepblue::*;
pub use pieces::*;

use crate::deepblue::Value;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Board([[(Piece, Color); 8]; 8]);

impl std::ops::Index<Pos> for Board {
    type Output = (Piece, Color);
    fn index(&self, p: Pos) -> &Self::Output {
        if p.is_invalid() {
            panic!("Invalid pos: {:?}", p)
        }
        unsafe {
            self.0
                .get_unchecked(p.x as usize)
                .get_unchecked(p.y as usize)
        }
    }
}

impl std::ops::IndexMut<Pos> for Board {
    fn index_mut(&mut self, p: Pos) -> &mut Self::Output {
        if p.is_invalid() {
            panic!("Invalid pos: {:?}", p)
        }
        unsafe {
            self.0
                .get_unchecked_mut(p.x as usize)
                .get_unchecked_mut(p.y as usize)
        }
    }
}

impl Board {
    fn can_move(&self, piece: Piece, pos: Pos, color: Color) -> Vec<Pos> {
        let mut buffer: Vec<_> = pos
            .valid_moves(piece, color)
            .iter()
            .filter(|mv| !mv.is_invalid())
            .copied()
            .collect();

        if piece == Piece::Knight {
            return buffer
                .iter()
                .filter(|&&mv| self[mv].1 != color || self[mv].0 == Piece::None)
                .copied()
                .collect();
        }

        let mut dead_vecs: Vec<(f32, f32, f32, Color)> = vec![];

        let vector_comp = |pos: &Pos, mv: &Pos| -> (f32, f32, f32) {
            let mut dx = (pos.x - mv.x) as f32;
            let mut dy = (pos.y - mv.y) as f32;

            let len = ((dx.powi(2) + dy.powi(2)) as f32).sqrt();
            dx = if len == 0. { 0. } else { dx / len };
            dy = if len == 0. { 0. } else { dy / len };
            (dx.round(), dy.round(), len)
        };

        for &mv in &buffer {
            let (dx, dy, len) = vector_comp(&pos, &mv);
            if self[mv].0 != Piece::None {
                dead_vecs.push((dx, dy, len, self[mv].1));
            }
        }

        buffer = buffer
            .iter()
            .filter(move |mv| {
                if mv.x > 8 || mv.x < 0 || mv.y > 8 || mv.y < 0 {
                    return false;
                }
                let (dx, dy, len) = vector_comp(&pos, mv);
                // !dead_arrows.iter().any(|a| a(mv.x) == mv.y)
                !dead_vecs.iter().any(|(x, y, slen, scolor)| {
                    if *scolor == color || piece == Piece::Pawn {
                        *x == dx && *y == dy && len >= *slen
                    } else {
                        *x == dx && *y == dy && len > *slen
                    }
                })
            })
            .copied()
            .collect();

        if piece == Piece::Pawn {
            let mv_dir = if color == Black { -1 } else { 1 };
            let mv_pos = [
                Pos {
                    x: pos.x + 1,
                    y: pos.y + mv_dir,
                },
                Pos {
                    x: pos.x - 1,
                    y: pos.y + mv_dir,
                },
            ];

            for mv in mv_pos {
                if mv.is_invalid() {
                    continue;
                }
                let p = self[mv];
                if p.0 != Piece::None && p.1 != color {
                    buffer.push(mv)
                }
            }
        }

        buffer
    }

    // returns true and moves piece if move is valid. Else returns false and does not move piece.
    pub fn move_piece(&mut self, mv: Move) -> bool {
        if mv.0.is_invalid() || mv.1.is_invalid() {
            return false;
        }
        let p = self[mv.0];
        let can_move = self.can_move(p.0, mv.0, p.1);

        can_move.iter().any(|can_mv| {
            if *can_mv == mv.1 {
                self[mv.0].0 = Piece::None;
                self[mv.1] = p;
                true
            } else {
                false
            }
        })
    }

    #[cfg(not(target_family = "wasm"))]
    fn print(&self) {
        use crossterm::style::Stylize;

        println!("A B C D E F G H\n");

        for y in 0..8 {
            for x in 0..8 {
                use Piece::*;

                let mut c = (match self.0[x][y].0 {
                    Queen => "Q",
                    Knight => "K",
                    King => "#",
                    Bishop => "B",
                    Rook => "R",
                    Pawn => "P",
                    None => ".",
                }
                .to_string()
                    + " ")
                    .bold();

                if self.0[x][y].1 == White && self.0[x][y].0 != None {
                    c = c.negative();
                }

                print!("{}", c);
            }
            println!(" {}", y + 1);
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! gen_row {
	($color: ident, $($p: tt)*) => {
		[$(
            (match stringify!($p){
                "q" => Piece::Queen,
                "*" => Piece::King,
                "k" => Piece::Knight,
                "r" => Piece::Rook,
                "b" => Piece::Bishop,
                _ => Piece::None,
            }, $color)
        ),*]
	};
}

impl std::default::Default for Board {
    fn default() -> Self {
        let tmp = [
            gen_row![White, r k b * q b k r],
            [(Piece::Pawn, White); 8],
            [(Piece::None, White); 8],
            [(Piece::None, White); 8],
            [(Piece::None, White); 8],
            [(Piece::None, White); 8],
            [(Piece::Pawn, Black); 8],
            gen_row![Black, r k b * q b k r],
        ];

        let mut buffer = tmp.clone();

        for x in 0..8 {
            for y in 0..8 {
                buffer[x][y] = tmp[y][x]
            }
        }

        Self(buffer)
    }
}

#[wasm_bindgen]
pub fn default_board() -> GameState {
    GameState::default()
}

#[wasm_bindgen]
pub fn board_move(board: &mut GameState, a: Pos, b: Pos) -> bool {
    //let pos = Pos::from_u16(pos);
    board.move_piece((a, b))
}

#[wasm_bindgen]
pub fn board_is_valid_move(board: &mut GameState, a: Pos, b: Pos) -> bool {
    if a.is_invalid() || b.is_invalid() {
        return false;
    }
    let p = board.board[a];
    board
        .board
        .can_move(p.0, a, p.1)
        .iter()
        .any(|action| *action == b)
}

pub const DEFAULT_DEPTH: usize = 4;

// Main function for debugging
#[cfg(not(target_family = "wasm"))]
pub fn main() -> Result<()> {
    use Value::*;
    assert!(Num(0) > NegInf);
    assert!(Num(0) < Inf);
    assert!(NegInf < Inf);
    assert!(Inf > NegInf);
    //let mut board = [' '; 8 * 8];

    let mut game = deepblue::GameState::default();
    game.winner = None;

    //game.0[5][5].0 = Piece::Pawn;
    //game.0[5][5].1 = Black;
    //game.0[2][3].0 = Piece::Pawn;
    //game.0[2][6].0 = Piece::Pawn;
    //game.0[3][2].0 = Piece::Pawn;
    //let p = Piece::Pawn;
    //let px = 3;
    //let py = 1;

    //for pos in game.can_move(p, Pos { x: px, y: py }, White) {
    //    let pos = pos.to_u16().to_ne_bytes();
    //    board[(pos[0] + pos[1] * 8).min(63) as usize] = '.';
    //}

    let mut turn = Black;
    let mut board_value = Num(0);
    let mut round = 0;

    use std::io::BufRead;
    let stdin = std::io::stdin();
    let mut usr_in = stdin.lock().lines();

    while game.winner.is_none() && board_value != Inf && board_value != NegInf {
        turn.invert();
        board_value = game.board.naive_value(turn);
        println!(
            "\n\n Round {} - {turn:?} has {:?} points - {turn:?}'s turn",
            round,
            board_value,
            turn = turn
        );

        if turn == White {
            let action = game.best_move(turn, DEFAULT_DEPTH);
            game.move_piece(action);
        } else {
            loop {
                print!("your turn --> ");
                stdout().flush()?;
                let action_str = usr_in.next().unwrap()?.to_uppercase();
                let mut action_str = action_str.chars();
                //n - 41
                let a = Pos {
                    x: action_str.next().unwrap() as i8 - 65,
                    y: format!("{}", action_str.next().unwrap()).parse::<i8>()? - 1,
                };
                let b = Pos {
                    x: action_str.next().unwrap() as i8 - 65,
                    y: format!("{}", action_str.next().unwrap()).parse::<i8>()? - 1,
                };

                if game.move_piece((a, b)) {
                    break;
                }
            }
        }

        game.board.print();
        round += 1;
    }
    Ok(())
}

pub fn hello_world() {
    unsafe { log("hello, world!") };
}
