use std::collections::HashMap;

use crate::game;



#[derive(Debug)]
pub struct GameState {
    chess_games: HashMap<uuid::Uuid, game::ChessGame>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            chess_games: HashMap::new(),
        }
    }

    pub fn push_game(&mut self, id: uuid::Uuid, game:  game::ChessGame) {
        self.chess_games.insert(id, game);
    }

    pub fn game(&self, id: &uuid::Uuid) -> Option<&game::ChessGame> {
        self.chess_games.get(id)
    }

    pub fn get_game_or_create(&mut self, id: uuid::Uuid) -> &mut  game::ChessGame {
        self.chess_games.entry(id).or_insert_with( game::ChessGame::new)
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
