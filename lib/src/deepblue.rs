use crate::*;

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
