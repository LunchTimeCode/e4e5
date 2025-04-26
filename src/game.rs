

use crate::chess::ChessMove;

#[derive(Debug)]
pub struct ChessGame {
    moves: Vec<ChessMove>,
}

// todo move to game mod
impl ChessGame {
    pub fn new() -> Self {
        Self { moves: vec![] }
    }

    pub fn push_move(&mut self, mv: ChessMove) {
        self.moves.push(mv);
    }

    pub fn moves(&self) -> Vec<ChessMove> {
        self.moves.clone()
    }
}