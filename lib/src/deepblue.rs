use std::sync::{
    atomic::{AtomicI32, Ordering::*},
    Arc,
};

use crate::*;

#[derive(Clone, Copy, Default)]
pub struct GameState {
    pub board: Board,
    pub winner: Option<Color>,
}

impl GameState {
    pub fn force_recheck_winner(&mut self) {
        if !self
            .board
            .0
            .iter()
            .flatten()
            .any(|n| n.0 == Piece::King && n.1 == White)
        {
            self.winner = Some(Black);
        } else if !self
            .board
            .0
            .iter()
            .flatten()
            .any(|n| n.0 == Piece::King && n.1 == Black)
        {
            self.winner = Some(White);
        }
    }

    pub fn get_valid_moves(&self, color: Color) -> impl ParallelIterator<Item = Move> + '_ {
        (0..8 * 8)
            .into_par_iter()
            .filter_map(move |n| {
                let x = n % 8;
                let y = (n - x) / 8;
                if self.board.0[x][y].1 != color {
                    return None;
                }

                let pos = Pos {
                    x: x as i8,
                    y: y as i8,
                };
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

    pub fn best_move(&mut self, color: Color, depth: usize) -> Move {
        let alpha = Arc::new(AtomicI32::new(-i32::MAX));

        maxormin(
            self.get_valid_moves(color).map(|action| {
                let mut sim = *self;
                sim.board.move_piece(action);

                let alpha = alpha.clone();
                let value = sim.minimax(color.not(), depth, Num(alpha.load(SeqCst)), Inf);
                alpha.fetch_max(value.i32(), SeqCst);

                ValuedMove { value, action }
            }),
            color == White,
        )
        .unwrap()
        .action
    }

    pub fn minimax(&mut self, color: Color, depth: usize, alpha: Value, beta: Value) -> Value {
        let alpha = Arc::new(AtomicI32::new(alpha.i32()));
        let beta = Arc::new(AtomicI32::new(beta.i32()));

        if depth == 0 {
            return self.board.naive_value(White);
        }

        maxormin(
            self.get_valid_moves(color)
                .filter(|_| beta.load(SeqCst) > alpha.load(SeqCst))
                .map(|action| {
                    let mut sim = *self;
                    sim.board.move_piece(action);
                    let value = sim.minimax(
                        color.not(),
                        depth - 1,
                        Num(alpha.load(SeqCst)),
                        Num(beta.load(SeqCst)),
                    );
                    if color == White {
                        alpha.fetch_max(value.i32(), SeqCst);
                    } else {
                        alpha.fetch_min(value.i32(), SeqCst);
                    }
                    value
                }),
            color == White,
        )
        .unwrap_or(self.board.naive_value(White))
    }
}

fn maxormin<T: Ord>(i: impl ParallelIterator<Item = T>, is_max: bool) -> Option<T> {
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

impl Value {
    pub fn minus(&self, other: &Self) -> Self {
        match self {
            Inf => Inf,
            NegInf => NegInf,
            Num(l) => match other {
                Inf => NegInf,
                NegInf => Inf,
                Num(r) => Num(l - r),
            },
        }
    }
    pub fn i32(&self) -> i32 {
        match self {
            Inf => i32::MAX,
            NegInf => -i32::MAX,
            Num(n) => *n,
        }
    }
}

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
