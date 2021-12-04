pub use rayon::prelude::*;
use wasm_bindgen::prelude::*;

#[cfg(target_family = "wasm")]
pub use wasm_bindgen_rayon::init_thread_pool;

mod deepblue;
mod pieces;
pub use pieces::*;

use crate::deepblue::Value;

#[derive(Clone, Copy)]
pub struct Board([[(Piece, Color); 8]; 8]);

impl Board {
    fn can_move(&self, piece: Piece, pos: Pos, color: Color) -> Vec<Pos> {
        let buffer: Vec<_> = pos
            .valid_moves(piece, color)
            .iter()
            .filter(|mv| !mv.is_invalid())
            .copied()
            .collect();

        if piece == Piece::Knight {
            return buffer
                .iter()
                .filter(|mv| self.0[mv.x as usize][mv.y as usize].1 != color)
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

        for mv in &buffer {
            let (dx, dy, len) = vector_comp(&pos, mv);
            if self.0[mv.x as usize][mv.y as usize].0 != Piece::None {
                dead_vecs.push((dx, dy, len, self.0[mv.x as usize][mv.y as usize].1));
            }
        }

        buffer
            .iter()
            .filter(move |mv| {
                if mv.x > 8 || mv.x < 0 || mv.y > 8 || mv.y < 0 {
                    return false;
                }
                let (dx, dy, len) = vector_comp(&pos, mv);
                // !dead_arrows.iter().any(|a| a(mv.x) == mv.y)
                !dead_vecs.iter().any(|(x, y, slen, scolor)| {
                    if *scolor == color {
                        *x == dx && *y == dy && len >= *slen
                    } else {
                        *x == dx && *y == dy && len > *slen
                    }
                })
            })
            .copied()
            .collect()
    }

    // returns true and moves piece if move is valid. Else returns false and does not move piece.
    pub fn move_piece(&mut self, mv: Move) -> bool {
        if mv.0.is_invalid() || mv.1.is_invalid() {
            return false;
        }
        let p = self.0[mv.0.x as usize][mv.0.y as usize];
        let can_move = self.can_move(p.0, mv.0, p.1);

        can_move.iter().any(|can_mv| {
            if *can_mv == mv.1 {
                self.0[mv.0.x as usize][mv.0.y as usize].0 = Piece::None;
                self.0[mv.1.x as usize][mv.1.y as usize] = p;
                true
            } else {
                false
            }
        })
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
            gen_row![White, r k b q * b k r],
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

// Main function for debugging
pub fn main() {
    use Value::*;
    assert!(Num(0) > NegInf);
    assert!(Num(0) < Inf);
    assert!(NegInf < Inf);
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
    while game.winner.is_none() {
        turn.invert();
        println!("\n\n");

        let mv = game.simulate_til_win(turn, None, 0, 5);
        game.board.move_piece(mv.mv);

        println!("A  B  C  D  E  F  G  H\n");
        for y in 0..8 {
            for x in 0..8 {
                use Piece::*;
                print!(
                    "{}{} ",
                    match game.board.0[x][y].0 {
                        Queen => 'Q',
                        Knight => 'k',
                        King => '*',
                        Bishop => 'b',
                        Rook => 'r',
                        Pawn => 'p',
                        None => '.',
                    },
                    if game.board.0[x][y].0 == None {
                        ' '
                    } else if game.board.0[x][y].1 == White {
                        'w'
                    } else {
                        'b'
                    }
                );
            }
            println!(" {}", y);
        }
    }
}

pub fn hello_world() {
    unsafe { log("hello, world!") };
}
