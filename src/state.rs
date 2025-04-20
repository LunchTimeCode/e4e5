use std::collections::HashMap;

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

#[derive(Debug)]
pub struct GameState {
    chess_games: HashMap<uuid::Uuid, ChessGame>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            chess_games: HashMap::new(),
        }
    }

    pub fn push_game(&mut self, id: uuid::Uuid, game: ChessGame) {
        self.chess_games.insert(id, game);
    }

    pub fn game(&self, id: &uuid::Uuid) -> Option<&ChessGame> {
        self.chess_games.get(id)
    }

    pub fn get_game_or_create(&mut self, id: uuid::Uuid) -> &mut ChessGame {
        self.chess_games.entry(id).or_insert_with(ChessGame::new)
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            chess_games: HashMap::new(),
        }
    }
}

pub fn initial() -> shadow::_GameState {
    shadow::_GameState::default()
}

pub mod shadow {

    use std::sync::Arc;

    use rocket::tokio::sync::Mutex;

    use super::GameState;

    type LockedGameState = Arc<Mutex<GameState>>;

    pub struct _GameState {
        state: LockedGameState,
    }

    impl Default for _GameState {
        fn default() -> Self {
            _GameState {
                state: Arc::new(Mutex::new(GameState::default())),
            }
        }
    }

    impl _GameState {
        pub async fn get(&self) -> rocket::tokio::sync::MutexGuard<'_, GameState> {
            self.state.lock().await
        }
    }
}
