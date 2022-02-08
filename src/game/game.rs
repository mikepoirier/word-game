use std::fmt::Display;

use chrono::Utc;
use uuid::Uuid;

use crate::{AppResult, ApplicationError};

#[derive(Debug, Clone)]
struct Guess(Option<String>, Option<String>);

impl Guess {
    fn is_match(&self) -> bool {
        self.0 == self.1
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub id: String,
    pub start_time: i64,
    players: Vec<String>,
    guesses: Vec<Guess>,
}

impl Game {
    pub fn new(p1_username: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            start_time: Utc::now().timestamp(),
            players: vec![p1_username.to_string()],
            guesses: vec![],
        }
    }

    pub fn add_player(&mut self, username: &str) -> AppResult<()> {
        if self.players.len() >= 2 {
            return Err(ApplicationError::new(
                "Cannot add player",
                "Game is full",
                None
            ))
        }

        self.players.push(username.to_string());

        Ok(())
    }

    pub fn players(&self) -> Vec<String> {
        self.players.clone()
    }

    pub fn add_guess(&mut self, username: &str, guess: &str) -> AppResult<bool> {
        let idx = self.players.iter().position(|u| u == username).unwrap();

        match self.guesses.last_mut() {
            Some(last_guess) => {

                if last_guess.0.is_some() && last_guess.1.is_some() {
                    let guess_pair = if idx == 0 {
                        Guess(Some(guess.to_string()), None)
                    } else {
                        Guess(None, Some(guess.to_string()))
                    };
                    self.guesses.push(guess_pair);
                } else if last_guess.0.is_none() && idx == 0 {
                    last_guess.0 = Some(guess.to_string());
                } else if last_guess.1.is_none() && idx == 1 {
                    last_guess.1 = Some(guess.to_string());
                } else {
                    return Err(ApplicationError::new(
                        "guess already exists for this round",
                        "You have already guessed this round. Please wait until the other player has guessed!",
                        None
                    ))
                }
            },
            None => {
                let guess_pair = if idx == 0 {
                    Guess(Some(guess.to_string()), None)
                } else {
                    Guess(None, Some(guess.to_string()))
                };
                self.guesses.push(guess_pair);
            },
        }

        match self.guesses.last() {
            Some(guess_pair) => Ok(guess_pair.is_match()),
            None => Ok(false),
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WIP: Game {}", self.id)
    }
}
