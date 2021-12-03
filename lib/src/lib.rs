use wasm_bindgen::prelude::*;

mod deepblue;
mod pieces;
pub use pieces::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
struct Board([[(Piece, Color); 8]; 8]);

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

// Main function for debugging
pub fn main() {
    let mut board = [' '; 8 * 8];

    let mut game = Board([[(Piece::None, White); 8]; 8]);
    //game.0[5][5].0 = Piece::Pawn;
    //game.0[5][5].1 = Black;
    //game.0[2][3].0 = Piece::Pawn;
    //game.0[2][6].0 = Piece::Pawn;
    game.0[3][2].0 = Piece::Pawn;
    let p = Piece::Pawn;
    let px = 3;
    let py = 1;

    for pos in game.can_move(p, Pos { x: px, y: py }, White) {
        let pos = pos.to_u16().to_ne_bytes();
        board[(pos[0] + pos[1] * 8).min(63) as usize] = '.';
    }

    println!("A  B  C  D  E  F  G  H\n");
    for y in 0..8 {
        for x in 0..8 {
            print!(
                "{}{} ",
                board[x + y * 8],
                if game.0[x][y].0 == Piece::Pawn {
                    "<"
                } else if x as i8 == px && y as i8 == py {
                    "X"
                } else {
                    " "
                }
            );
        }
        println!(" {}", y);
    }
}

pub fn hello_world() {
    unsafe { log("hello, world!") };
}
