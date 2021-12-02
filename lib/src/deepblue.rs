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
