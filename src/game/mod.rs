pub mod player;
pub mod game;

use std::{
    sync::{Arc, Mutex},
};

use crate::{database::DatabaseEnum, AppResult, ApplicationError};

use self::{player::{Player, PlayerStatus}, game::Game};

pub struct WordGame {
    database: Arc<Mutex<DatabaseEnum>>,
}

impl WordGame {
    pub fn new(database: Arc<Mutex<DatabaseEnum>>) -> Self {
        Self { database }
    }

    fn create_player(&mut self, username: &str) -> AppResult<Player> {
        let player = Player::new(username);
        self.save_player(&player)?;
        Ok(player)
    }

    pub fn new_game(&mut self, username: &str) -> AppResult<String> {
        let game = Game::new(username);
        let mut player = self.find_player(username)?;
        player.set_status(PlayerStatus::InGame{ game_id: game.id.clone() });
        self.save_player(&player)?;
        self.save_game(&game)?;
        Ok(game.id)
    }

    pub fn join_game(&mut self, username: &str, game_id: &str) -> AppResult<()> {
        let mut player = self.find_player(username)?;
        let mut game = self.find_game(game_id)?;

        game.add_player(username)?;
        player.set_status(PlayerStatus::InGame{ game_id: game.id.clone() });
        self.save_player(&player)?;
        self.save_game(&game)?;

        Ok(())
    }

    pub fn get_players_in_game(&self, game_id: &str) -> AppResult<Vec<String>> {
        let game = self.find_game(game_id)?;
        Ok(game.players())
    }

    pub fn debug(&self) {
        let db = self.database.clone();
        let db = db.lock().unwrap();
        println!("Players: {:?}", db.get_players());
        println!("Games: {:?}", db.get_games());
    }

    pub fn get_player(&mut self, username: &str) -> AppResult<Player> {
        if !self.has_player(username) {
            self.create_player(username)?;
        }
        self.find_player(username)
    }

    fn find_player(&self, username: &str) -> AppResult<Player> {
        let db = self.database.clone();
        let db = db.lock().unwrap();
        let player = db
            .get_player_by_username(username)?
            .ok_or(ApplicationError::new(
                "player not found",
                "Could not find player",
                None,
            ))?;
        Ok(player)
    }

    pub fn change_player_to_introducing(&mut self, username: &str) -> AppResult<Player> {
        let mut player = self.get_player(username)?;
        player.set_status(PlayerStatus::Introducing);
        self.save_player(&player)?;
        Ok(player)
    }

    pub fn introduce_player(&mut self, username: &str, display_name: &str) -> AppResult<Player> {
        let mut player = self.get_player(username)?;
        player.set_display_name(display_name);
        player.set_status(PlayerStatus::InLobby);
        self.save_player(&player)?;
        Ok(player)
    }

    pub fn return_to_lobby(&mut self, username: &str) -> AppResult<Player> {
        let mut player = self.get_player(username)?;
        player.set_status(PlayerStatus::InLobby);
        self.save_player(&player)?;
        Ok(player)
    }

    pub fn has_player(&self, username: &str) -> bool {
        let db = self.database.clone();
        let db = db.lock().unwrap();
        match db.get_player_by_username(username) {
            Ok(Some(_)) => true,
            _ => false
        }
    }

    pub fn find_game(&self, game_id: &str) -> AppResult<Game> {
        let db = self.database.clone();
        let db = db.lock().unwrap();
        let game = db.get_game(game_id)?.ok_or(ApplicationError::new(
            "game not found",
            "Could not find game",
            None,
        ))?;
        Ok(game)
    }

    fn save_game(&self, game: &Game) -> AppResult<()> {
        let db = self.database.clone();
        let mut db = db.lock().unwrap();
        db.save_game(game.clone())?;
        Ok(())
    }

    pub fn save_player(&self, player: &Player) -> AppResult<()> {
        let db = self.database.clone();
        let mut db = db.lock().unwrap();
        db.save_player(player.clone())?;
        Ok(())
    }

    pub fn submit_guess(&self, username: &str, game_id: &str, guess: &str) -> AppResult<bool> {
        let mut game = self.find_game(game_id)?;
        let win = game.add_guess(username, guess)?;
        self.save_game(&game)?;
        Ok(win)
    }
}
