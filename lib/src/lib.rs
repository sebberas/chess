use wasm_bindgen::prelude::*;

mod pieces;
pub use pieces::*;

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
        let mut dead_vecs: Vec<(f32, f32, f32)> = vec![];

        let vector_comp = |pos: &Pos, mv: &Pos| -> (f32, f32, f32) {
            let mut dx = (pos.x - mv.x) as f32;
            let mut dy = (pos.y - mv.y) as f32;

            let len = ((dx.powi(2) + dy.powi(2)) as f32).sqrt();
            dx = if len == 0. { 0. } else { dx / len };
            dy = if len == 0. { 0. } else { dy / len };
            (dx.floor(), dy.floor(), len.floor())
        };

        for mv in &buffer {
            let (dx, dy, len) = vector_comp(&pos, mv);
            if self.0[mv.x as usize][mv.y as usize].0 != Piece::None {
                dead_vecs.push((dx, dy, len));
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
                !dead_vecs
                    .iter()
                    .any(|(x, y, slen)| *x == dx && *y == dy && len > *slen)
            })
            .copied()
            .collect()
    }
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

    let mut game = Board([[(Piece::None, White); 8]; 8]);
    game.0[5][5].0 = Piece::Pawn;
    game.0[2][3].0 = Piece::Pawn;
    game.0[2][6].0 = Piece::Pawn;
    let p = Piece::Queen;

    for pos in game.can_move(p, Pos { x: 3, y: 5 }, White) {
        let pos = pos.to_u16().to_ne_bytes();
        board[(pos[0] + pos[1] * 8).min(63) as usize] = '.';
    }

    println!("A  B  C  D  E  F  G  H\n");
    for y in 0..8 {
        for x in 0..8 {
            print!(
                "{}{} ",
                board[x + y * 8],
                if game.0[y][x].0 == Piece::Pawn {
                    "<"
                } else if x == 3 && y == 5 {
                    "Q"
                } else {
                    " "
                }
            );
        }
        println!(" {}", y);
    }
}
