use crate::*;

fn winner(&Board) -> Option<Color> {
    if !self.0.iter().flatten().any(|n| n.1 == White) {
        return Some(Black);
    } else if !self.0.iter().flatten().any(|n| n.1 == Black) {
        return Some(White);
    }
    None
}
