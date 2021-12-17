#![feature(test)]

// # TODO inden aflevering
// - [ ] https://github.com/w3reality/wasm-mt
// - [ ] Move get_unchecked into index impl
// - [ ] Move wasm api stuff into seperate module

use std::io::{stdout, Write};

use anyhow::*;
use wasm_bindgen::prelude::*;

//#[cfg(target_family = "wasm")]
//pub use wasm_bindgen_rayon::init_thread_pool;

mod deepblue;
mod pieces;
pub use deepblue::*;
pub use pieces::*;
pub mod bench;

use crate::deepblue::Value;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Board([[(Piece, Color); 8]; 8]);

impl Board {
    pub unsafe fn get_unchecked(&self, p: Pos) -> &(Piece, Color) {
        self.0
            .get_unchecked(p.x as usize)
            .get_unchecked(p.y as usize)
    }

    pub unsafe fn get_unchecked_mut(&mut self, p: Pos) -> &mut (Piece, Color) {
        self.0
            .get_unchecked_mut(p.x as usize)
            .get_unchecked_mut(p.y as usize)
    }
}

//impl std::ops::Index<Pos> for Board {
//    type Output = (Piece, Color);
//    fn index(&self, p: Pos) -> &Self::Output {
//        if p.is_invalid() {
//            panic!("Invalid pos: {:?}", p)
//        }
//        unsafe { self.get_unchecked(p) }
//    }
//}
//
//impl std::ops::IndexMut<Pos> for Board {
//    fn index_mut(&mut self, p: Pos) -> &mut Self::Output {
//        if p.is_invalid() {
//            panic!("Invalid pos: {:?}", p)
//        }
//        unsafe { self.get_unchecked_mut(p) }
//    }
//}

mod sqrts;
impl Board {
    fn can_move(&self, piece: Piece, pos: Pos, color: Color) -> Vec<Pos> {
        use sqrts::*;

        let mut buffer: Vec<_> = pos
            .valid_moves(piece, color)
            .iter()
            .filter(|mv| !mv.is_invalid())
            .copied()
            .collect();

        if piece == Piece::Knight {
            return buffer
                .iter()
                .filter(|&&mv| unsafe {
                    self.get_unchecked(mv).1 != color || self.get_unchecked(mv).0 == Piece::None
                })
                .copied()
                .collect();
        }

        let mut dead_vecs: Vec<(i8, i8, i8, Color)> = vec![];

        let vector_comp = |pos: &Pos, mv: &Pos| -> (i8, i8, i8) {
            let dx = pos.x - mv.x;
            let dy = pos.y - mv.y;

            let mut comps = unsafe {
                // The magic of vector component lookup tables (which is something i just made up)
                // lets us avoid wasting cpu time on calculating the square root of the same number over and over again.
                *COMPS
                    .get_unchecked(dx.unsigned_abs() as usize)
                    .get_unchecked(dy.unsigned_abs() as usize)
            };
            comps.0 *= dx.signum(); // Sign copying so we don't need a big symetrical lookup table.
            comps.1 *= dy.signum();
            comps
        };

        for &mv in &buffer {
            let (dx, dy, len) = vector_comp(&pos, &mv);
            unsafe {
                if self.get_unchecked(mv).0 != Piece::None {
                    dead_vecs.push((dx, dy, len, self.get_unchecked(mv).1));
                }
            }
        }

        buffer = buffer
            .iter()
            .filter(move |mv| {
                if mv.x > 8 || mv.x < 0 || mv.y > 8 || mv.y < 0 {
                    return false;
                }
                let (dx, dy, len) = vector_comp(&pos, mv);
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
                let p = unsafe { self.get_unchecked(mv) };
                if p.0 != Piece::None && p.1 != color {
                    buffer.push(mv)
                }
            }
        }

        buffer
    }

    //fn can_move_slow(&self, piece: Piece, pos: Pos, color: Color) -> Vec<Pos> {
    //    let mut buffer: Vec<_> = pos
    //        .valid_moves(piece, color)
    //        .iter()
    //        .filter(|mv| !mv.is_invalid())
    //        .copied()
    //        .collect();

    //    if piece == Piece::Knight {
    //        return buffer
    //            .iter()
    //            .filter(|&&mv| unsafe {
    //                self.get_unchecked(mv).1 != color || self.get_unchecked(mv).0 == Piece::None
    //            })
    //            .copied()
    //            .collect();
    //    }

    //    let mut dead_vecs: Vec<(f32, f32, f32, Color)> = vec![];

    //    let vector_comp = |pos: &Pos, mv: &Pos| -> (f32, f32, f32) {
    //        let mut dx = (pos.x - mv.x) as f32;
    //        let mut dy = (pos.y - mv.y) as f32;

    //        let len = ((dx.powi(2) + dy.powi(2)) as f32).sqrt();
    //        dx = if len == 0. { 0. } else { dx / len };
    //        dy = if len == 0. { 0. } else { dy / len };
    //        (dx.round(), dy.round(), len)
    //    };

    //    for &mv in &buffer {
    //        let (dx, dy, len) = vector_comp(&pos, &mv);
    //        unsafe {
    //            if self.get_unchecked(mv).0 != Piece::None {
    //                dead_vecs.push((dx, dy, len, self.get_unchecked(mv).1));
    //            }
    //        }
    //    }

    //    buffer = buffer
    //        .iter()
    //        .filter(move |mv| {
    //            if mv.x > 8 || mv.x < 0 || mv.y > 8 || mv.y < 0 {
    //                return false;
    //            }
    //            let (dx, dy, len) = vector_comp(&pos, mv);
    //            !dead_vecs.iter().any(|(x, y, slen, scolor)| {
    //                if *scolor == color || piece == Piece::Pawn {
    //                    *x == dx && *y == dy && len >= *slen
    //                } else {
    //                    *x == dx && *y == dy && len > *slen
    //                }
    //            })
    //        })
    //        .copied()
    //        .collect();

    //    if piece == Piece::Pawn {
    //        let mv_dir = if color == Black { -1 } else { 1 };
    //        let mv_pos = [
    //            Pos {
    //                x: pos.x + 1,
    //                y: pos.y + mv_dir,
    //            },
    //            Pos {
    //                x: pos.x - 1,
    //                y: pos.y + mv_dir,
    //            },
    //        ];

    //        for mv in mv_pos {
    //            if mv.is_invalid() {
    //                continue;
    //            }
    //            let p = unsafe { self.get_unchecked(mv) };
    //            if p.0 != Piece::None && p.1 != color {
    //                buffer.push(mv)
    //            }
    //        }
    //    }

    //    buffer
    //}

    // returns true and moves piece if move is valid. Else returns false and does not move piece.
    pub fn move_piece(&mut self, mv: Move) -> bool {
        if mv.0.is_invalid() || mv.1.is_invalid() {
            return false;
        }
        let p = unsafe { *self.get_unchecked(mv.0) };
        let can_move = self.can_move(p.0, mv.0, p.1);

        can_move.iter().any(|can_mv| {
            if *can_mv == mv.1 {
                unsafe {
                    self.get_unchecked_mut(mv.0).0 = Piece::None;
                    *(self.get_unchecked_mut(mv.1)) = p;
                }
                true
            } else {
                false
            }
        })
    }

    #[cfg(not(target_family = "wasm"))]
    fn print(&self, debug_points: Vec<Pos>) {
        use std::slice::SliceIndex;

        use crossterm::style::Stylize;

        println!("A B C D E F G H\n");

        for y in 0..8 {
            for x in 0..8 {
                use Piece::*;
                let pos = Pos { x, y };

                let c = match unsafe { self.get_unchecked(pos).0 } {
                    Queen => "Q",
                    Knight => "K",
                    King => "#",
                    Bishop => "B",
                    Rook => "R",
                    Pawn => "P",
                    None => ".",
                };

                let s = format!(
                    "{}{}",
                    c,
                    if debug_points.iter().any(|mv| *mv == pos) {
                        "<".red().bold()
                    } else {
                        " ".bold()
                    }
                );

                if unsafe {
                    self.get_unchecked(pos).1 == White && self.get_unchecked(pos).0 != None
                } {
                    print!("{}", s.negative());
                } else {
                    print!("{}", s);
                }
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
    let p = unsafe { board.board.get_unchecked(a) };
    board
        .board
        .can_move(p.0, a, p.1)
        .iter()
        .any(|action| *action == b)
}

#[wasm_bindgen]
pub fn new_pos(x: i8, y: i8) -> Pos {
    Pos { x, y }
}

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
        //println!(
        //    "\n\n Round {} - {turn:?} has {:?} points - {turn:?}'s turn",
        //    round,
        //    board_value,
        //    turn = turn
        //);

        //if turn == White {
        let action = game.best_move(turn, 3);
        //unsafe {
        //    let slow =
        //        game.board
        //            .can_move_slow(game.board.get_unchecked(action.0).0, action.0, turn);
        //    let fast = game
        //        .board
        //        .can_move(game.board.get_unchecked(action.0).0, action.0, turn);
        //    if fast != slow {
        //        println!(
        //            "{}\nFAST != SLOW, when moving {:?} {}{}",
        //            "_".repeat(20),
        //            game.board.get_unchecked(action.0),
        //            "ABCDEFGH".as_bytes()[action.0.x as usize] as char,
        //            action.0.y + 1
        //        );

        //        println!("FAST:");
        //        game.board.print(fast);

        //        println!("\nSLOW:");
        //        game.board.print(slow);
        //        println!("{}", "_".repeat(20));
        //    }
        //}
        game.move_piece(action);
        //} else {
        //    loop {
        //        print!("your turn --> ");
        //        stdout().flush()?;
        //        let action_str = usr_in.next().unwrap()?.to_uppercase();
        //        let mut action_str = action_str.chars();
        //        //n - 41
        //        let a = Pos {
        //            x: action_str.next().unwrap() as i8 - 65,
        //            y: format!("{}", action_str.next().unwrap()).parse::<i8>()? - 1,
        //        };
        //        let b = Pos {
        //            x: action_str.next().unwrap() as i8 - 65,
        //            y: format!("{}", action_str.next().unwrap()).parse::<i8>()? - 1,
        //        };

        //        if game.move_piece((a, b)) {
        //            break;
        //        }
        //    }
        //}

        //game.board.print(None);
        round += 1;
    }
    Ok(())
}

pub fn hello_world() {
    unsafe { log("hello, world!") };
}
