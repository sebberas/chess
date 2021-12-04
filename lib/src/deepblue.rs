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

    pub fn get_valid_moves(&self, color: Color) -> Vec<Move> {
        let mut buffer = vec![];
        for y in 0..8 {
            for x in 0..8 {
                let piece = self.board.0[x][y];
                let pos = Pos {
                    x: x as i8,
                    y: y as i8,
                };

                if piece.1 == color {
                    for mv in self.board.can_move(piece.0, pos, color) {
                        buffer.push((pos, mv))
                    }
                }
            }
        }
        buffer
    }

    pub fn simulate_til_win(
        &mut self,
        color: Color,
        last_move: Option<Move>,
        depth: usize,
        max_depth: usize,
    ) -> ValuedMove {
        self.force_recheck_winner();

        let try_get_last_move = || {
            if let Some(mv) = last_move {
                mv
            } else {
                panic!("Last move is None")
            }
        };

        if let Some(winner) = self.winner {
            if last_move.is_none() {
                panic!("Wut? {}", if depth == 0 { "Won instant" } else { "" })
            }
            let mv = try_get_last_move();
            return if winner == color {
                ValuedMove { mv, value: Inf }
            } else {
                ValuedMove { mv, value: NegInf }
            };
        }

        if depth >= max_depth {
            return ValuedMove {
                mv: try_get_last_move(),
                value: self.board.naive_value(color),
            };
        }

        {
            let v = self.board.naive_value(color);
            if v < Num(-200) {
                return ValuedMove {
                    mv: try_get_last_move(),
                    value: v,
                };
            }
        }

        //let mut max_val = NegInf;
        //let mut best_move = (Pos { x: 0, y: 0 }, Pos { x: 0, y: 0 });

        let ValuedMove {
            mv: best_move,
            value: max_val,
        } = self
            .get_valid_moves(color)
            .par_iter()
            .map(|mv| {
                let mut sim = self.clone();
                sim.board.move_piece(*mv);
                sim.simulate_til_win(color.not(), Some(*mv), depth + 1, max_depth)
                // TODO: color.not() is wrong. This might be the time to implement alpha-beta pruning.
            })
            .max()
            .unwrap();

        ValuedMove {
            mv: if let Some(mv) = last_move {
                mv
            } else {
                best_move
            },
            value: max_val,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ValuedMove {
    pub mv: Move,
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

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
pub enum Value {
    NegInf,
    Num(i32),
    Inf,
}

use Value::*;

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
        let mut val = 0;
        let mut sk = false; // Does 'color' have a king?
        let mut ok = false; // Does other color have a king?

        for p in self.0.iter().flatten() {
            if p.1 == color {
                sk = p.0 == Piece::King || sk;
                val += piece_value(p.0);
            } else {
                ok = p.0 == Piece::King || ok;
                val -= piece_value(p.0);
            }
        }

        if !sk {
            NegInf
        } else if !ok {
            Inf
        } else {
            Num(val)
        }
    }
}
