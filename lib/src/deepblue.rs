use crate::*;

#[derive(Clone, Copy)]
struct GameState {
    board: Board,
    winner: Option<Color>,
}

impl GameState {
    fn force_recheck_winner(&mut self) {
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

    fn get_valid_moves(&self, color: Color) -> Vec<Move> {
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

    fn simulate_til_win(
        &mut self,
        color: Color,
        last_move: Option<Move>,
        depth: usize,
        max_depth: usize,
    ) -> (Move, Value) {
        self.force_recheck_winner();

        let try_get_last_move = || {
            if let Some(mv) = last_move {
                mv
            } else {
                panic!("Cannot simulate a game that already has a winner.")
            }
        };

        if let Some(winner) = self.winner {
            let mv = try_get_last_move();
            return if winner == color {
                (mv, Inf)
            } else {
                (mv, NegInf)
            };
        }

        if depth >= max_depth {
            return (try_get_last_move(), self.board.naive_value(color));
        }

        let mut max_val = NegInf;
        let mut best_move = (Pos { x: 0, y: 0 }, Pos { x: 0, y: 0 });

        for mv in self.get_valid_moves(color) {
            let mut sim = self.clone();
            sim.board.move_piece(mv);
            let (mv, val) = sim.simulate_til_win(color, Some(mv), depth + 1, max_depth);
            if val > max_val {
                max_val = val;
                best_move = mv;
            }
        }

        (best_move, max_val)
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
enum Value {
    NegInf,
    Num(i32),
    Inf,
}

use Value::*;

fn piece_value(p: Piece) -> i32 {
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
    fn naive_value(&self, color: Color) -> Value {
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
