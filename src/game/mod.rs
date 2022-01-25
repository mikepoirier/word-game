use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

use crate::{database::DatabaseEnum, AppResult, ApplicationError, time::duration::FormattedDuration};
use chrono::prelude::*;
use uuid::Uuid;

pub struct WordGame {
    database: Arc<Mutex<DatabaseEnum>>,
}

impl WordGame {
    pub fn new(database: Arc<Mutex<DatabaseEnum>>) -> Self {
        Self { database }
    }

    pub fn create_player(&mut self, username: &str, display_name: &str) -> AppResult<Player> {
        let player = Player::new(username, display_name);
        self.save_player(&player)?;
        Ok(player)
    }

    //Create Game
    pub fn create_game(&mut self) -> AppResult<Game> {
        let game = Game::new();
        self.save_game(&game)?;
        Ok(game)
    }

    //Join game
    pub fn join_game(&mut self, username: &str, game_id: &str) -> AppResult<()> {
        let mut player = self.get_player(username)?;
        let mut game = self.get_game(game_id)?;
        let game_id = game.id.clone();

        if game.player_1_username == None {
            game.player_1_username = Some(player.username.clone());
        } else if game.player_2_username == None {
            game.player_2_username = Some(player.username.clone());
        } else {
            return Err(ApplicationError::new(
                "cannot join error",
                "Game is full",
                None,
            ));
        }
        self.save_game(&game)?;

        player.current_game_id = Some(game_id);
        self.save_player(&player)?;

        Ok(())
    }

    pub fn debug(&self) {
        let db = self.database.clone();
        let db = db.lock().unwrap();
        println!("Players: {:?}", db.get_players());
        println!("Games: {:?}", db.get_games());
    }

    // //Leave game
    // pub fn leave_current_game(&mut self, username: &str) {
    //     todo!("Not Implemented")
    // }

    //Guess
    pub fn submit_guess(&mut self, username: &str, guess: &str) -> AppResult<()> {
        let player = self.get_player(username)?;
        let current_game = player.current_game_id.ok_or(ApplicationError::new(
            "no current game",
            "Player does not have a current game",
            None,
        ))?;
        let mut game = self.get_game(&current_game)?;

        if game.guesses.len() <= game.current_round {
            game.guesses.push((None, None));
        }

        if Some(username.to_string()) == game.player_1_username {
            if game.guesses[game.current_round].0 == None {
                game.guesses[game.current_round].0 = Some(guess.into());
            } else {
                return Err(ApplicationError::new(
                    "already guessed",
                    "Player already guessed for this round",
                    None,
                ));
            }
        } else if Some(username.to_string()) == game.player_2_username {
            if game.guesses[game.current_round].1 == None {
                game.guesses[game.current_round].1 = Some(guess.into());
            } else {
                return Err(ApplicationError::new(
                    "already guessed",
                    "Player already guessed for this round",
                    None,
                ));
            }
        }

        let p1_guess = game.guesses[game.current_round]
            .0
            .as_ref()
            .map(|g| g.to_lowercase());
        let p2_guess = game.guesses[game.current_round]
            .1
            .as_ref()
            .map(|g| g.to_lowercase());

        if p1_guess == p2_guess {
            game.complete = true;
            game.end_time = Some(Utc::now().timestamp());
        }

        if p1_guess != None && p2_guess != None {
            game.current_round += 1;
        }

        self.save_game(&game)?;

        Ok(())
    }

    pub fn is_game_complete(&self, game_id: &str) -> AppResult<bool> {
        let game = self.get_game(game_id)?;

        Ok(game.complete)
    }

    pub fn get_guesses(&self, game_id: &str) -> AppResult<Vec<Guess>> {
        let game = self.get_game(game_id)?;

        Ok(game.guesses)
    }

    fn get_player(&self, username: &str) -> AppResult<Player> {
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

    pub fn get_game(&self, game_id: &str) -> AppResult<Game> {
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

    fn save_player(&self, player: &Player) -> AppResult<()> {
        let db = self.database.clone();
        let mut db = db.lock().unwrap();
        db.save_player(player.clone())?;
        Ok(())
    }
    // //Force Win
    // pub fn force_win(&mut self, username: &str, game_id: &str) {
    //     todo!("Not Implemented")
    // }

    // //Player Status
    // pub fn get_player_status(&self, username: &str) {
    //     todo!("Not Implemented")
    // }

    // //Statistics
    // pub fn statistics(&self) {
    //     todo!("Not Implemented")
    // }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub username: String,
    pub display_name: String,
    current_game_id: Option<String>,
}

impl Player {
    pub fn new(username: &str, display_name: &str) -> Self {
        Self {
            username: username.into(),
            display_name: display_name.into(),
            current_game_id: None,
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name)
    }
}

pub type Guess = (Option<String>, Option<String>);

#[derive(Debug, Clone)]
pub struct Game {
    pub id: String,
    start_time: i64,
    end_time: Option<i64>,
    complete: bool,
    current_round: usize,
    player_1_username: Option<String>,
    player_2_username: Option<String>,
    guesses: Vec<Guess>,
}

impl Game {
    fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            start_time: Utc::now().timestamp(),
            end_time: None,
            complete: false,
            current_round: 0,
            player_1_username: None,
            player_2_username: None,
            guesses: vec![],
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut guesses = String::new();
        let p1 = self
            .player_1_username
            .as_ref()
            .map_or("???", |s| s.as_str());
        let p2 = self
            .player_2_username
            .as_ref()
            .map_or("???", |s| s.as_str());
        guesses.push_str(p1);
        guesses.push('\t');
        guesses.push_str(p2);
        guesses.push('\n');
        self.guesses.iter().for_each(|g| {
            if let Some(g1) = &g.0 {
                guesses.push_str(g1);
            }
            if let Some(g2) = &g.1 {
                guesses.push('\t');
                guesses.push_str(g2);
            }
            guesses.push('\n');
        });
        let start = Utc.timestamp(self.start_time, 0);
        let end = match self.end_time {
            Some(end) => Utc.timestamp(end, 0),
            None => Utc::now(),
        };
        let duration = end - start;
        let duration = FormattedDuration::from(duration);

        write!(f, "Guesses:\n{}\nDuration: {}", guesses.trim(), duration)
    }
}
