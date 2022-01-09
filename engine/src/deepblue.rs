use std::{cell::RefCell, ops::Not};

use crate::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Default)]
pub struct GameState {
    pub board: Board,
    pub winner: Option<Color>,
}

impl GameState {
    pub fn get_valid_moves(&self, color: Color) -> impl Iterator<Item = Move> + '_ {
        (0..8 * 8)
            .filter_map(move |n| {
                let x = n % 8;
                let y = n / 8;
                let pos = Pos {
                    x: x as i8,
                    y: y as i8,
                };

                unsafe {
                    let p = self.board.get_unchecked(pos);
                    if p.1 != color || p.0 == Piece::None {
                        return None;
                    }
                }

                let piece = self.board.0[x][y].0;
                Some(
                    self.board
                        .can_move(piece, pos, color)
                        .iter()
                        .map(|action| (pos, *action))
                        .collect::<Vec<_>>(),
                )
            })
            .flatten()
    }

    // Finder det bedste move.
    pub fn best_move(&mut self, color: Color, depth: usize) -> Move {
        let mut alpha = NegInf;
        let mut beta = Inf;

        maxormin(
            self.get_valid_moves(color).map(|action| {
                let mut sim = *self;
                sim.move_piece(action);

                let value = sim.minimax(color.not(), depth, alpha, beta);
                if color == White {
                    alpha = alpha.max(value);
                } else {
                    beta = beta.min(value);
                }

                ValuedMove { value, action }
            }),
            color == White,
        )
        .unwrap()
        .action
    }

    pub fn minimax(
        &mut self,
        color: Color,
        depth: usize,
        mut alpha: Value,
        mut beta: Value,
    ) -> Value {
        if let Some(winner) = self.winner {
            return if winner == White { Inf } else { NegInf };
        }
        if depth == 0 {
            return self.board.naive_value(White);
        }

        let is_done = RefCell::new(false);

        maxormin(
            self.get_valid_moves(color)
                .filter(|_| is_done.borrow().not())
                .map(|action| {
                    let mut sim = *self;
                    sim.move_piece(action);
                    let value = sim.minimax(color.not(), depth - 1, alpha, beta);
                    if color == White {
                        alpha = alpha.max(value);
                    } else {
                        beta = beta.min(value);
                    }
                    if alpha > beta {
                        let mut is_done = is_done.borrow_mut();
                        *is_done = true;
                    }
                    value
                }),
            color == White,
        )
        .unwrap_or(self.board.naive_value(White))
    }

    pub fn move_piece(&mut self, action: Move) -> bool {
        if action.0.is_invalid() || action.1.is_invalid() {
            return false;
        }
        let won = unsafe { self.board.get_unchecked(action.1).0 == Piece::King };
        let losser = unsafe { self.board.get_unchecked(action.1).1 };
        let moved = self.board.move_piece(action);

        if moved && won {
            self.winner = Some(losser.not())
        }
        moved
    }
}

fn maxormin<T: Ord>(i: impl Iterator<Item = T>, is_max: bool) -> Option<T> {
    if is_max {
        i.max()
    } else {
        i.min()
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ValuedMove {
    pub action: Move,
    pub value: Value,
}

impl PartialOrd for ValuedMove {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for ValuedMove {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Debug)]
pub enum Value {
    NegInf,
    Num(i32),
    Inf,
}

use Value::*;

// Værdierne for de forskellige brikker.
// Dette bruges af den kunstige intelligens til at udregne hvilke brikker den skal tage.
pub fn piece_value(p: Piece) -> i32 {
    use Piece::*;
    match p {
        Pawn => 1,
        Knight => 3,
        Bishop => 3,
        Rook => 5,
        Queen => 9,
        King => 100,
        None => 0,
    }
}

impl Board {
    pub fn naive_value(&self, color: Color) -> Value {
        let mut a = 0;
        let mut b = 0;
        let mut sk = false; // Does 'color' have a king?
        let mut ok = false; // Does other color have a king?

        for p in self.0.iter().flatten() {
            if p.1 == color {
                sk = p.0 == Piece::King || sk;
                a += piece_value(p.0);
            } else {
                ok = p.0 == Piece::King || ok;
                b += piece_value(p.0);
            }
        }

        if !sk {
            NegInf
        } else if !ok {
            Inf
        } else {
            Num(a - b)
        }
    }
}

#[wasm_bindgen]
// Retunerer en flad liste der indeholder det bedste move. Den retunerede array: [x1, y1, x2, y2].
pub fn get_best_move(board: Board, color: Color, depth: usize) -> Box<[i8]> {
    let mut game = GameState {
        board,
        winner: None,
    };
    let bm = game.best_move(color, depth);
    Box::new([bm.0.x, bm.0.y, bm.1.x, bm.1.y])
}
