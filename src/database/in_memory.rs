use std::collections::HashMap;

use crate::{game::{Player, Game}, AppResult};

use super::Database;

pub struct InMemoryDatabase {
    players: HashMap<String, Player>,
    games: HashMap<String, Game>
}

impl InMemoryDatabase {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            games: HashMap::new()
        }
    }
}

impl Default for InMemoryDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl Database for InMemoryDatabase {
    fn save_player(&mut self, player: Player) -> AppResult<()> {
        self.players.insert(player.username.clone(), player);

        Ok(())
    }

    fn get_players(&self) -> AppResult<Vec<Player>> {
        Ok(self.players.values().cloned().collect())
    }

    fn get_player_by_username(&self, username: &str) -> AppResult<Option<Player>> {
        Ok(self.players.get(username).cloned())
    }

    fn save_game(&mut self, game: crate::game::Game) -> AppResult<()> {
        self.games.insert(game.id.clone(), game);

        Ok(())
    }

    fn get_games(&self) -> AppResult<Vec<Game>> {
        Ok(self.games.values().cloned().collect())
    }

    fn get_game(&self, game_id: &str) -> AppResult<Option<Game>> {
        Ok(self.games.get(game_id).cloned())
    }
}