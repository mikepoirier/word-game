use std::{io::{stdout, Write, stdin}, sync::{Arc, Mutex}};
use rpassword::prompt_password_stdout;
use async_trait::async_trait;

use crate::{game::WordGame, AppResult, ApplicationError};

use super::Runner;

pub struct ConsoleRunner {

}

impl ConsoleRunner {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ConsoleRunner {
    fn default() -> Self {
        Self::new()
    }
}

const PLAYER_1_USERNAME: &str = "player1";
const PLAYER_2_USERNAME: &str = "player2";

#[async_trait]
impl Runner for ConsoleRunner {
    async fn run(&self, word_game: Arc<Mutex<WordGame>>) -> AppResult<()> {
        println!("Welcome to the word game!");
        let player_1_name = prompt("Enter player 1's name:");
        let word_game = word_game;
        let mut word_game = word_game.lock().unwrap();
        let p1 = word_game.create_player(PLAYER_1_USERNAME, player_1_name.as_str())?;
        let player_2_name = prompt("Enter player 2's name:");
        let p2 = word_game.create_player(PLAYER_2_USERNAME, player_2_name.as_str())?;
        
        let game = word_game.create_game()?;
        let game_id = game.id;
        word_game.join_game(PLAYER_1_USERNAME, &game_id)?;
        word_game.join_game(PLAYER_2_USERNAME, &game_id)?;

        while !word_game.is_game_complete(&game_id)? {
            let p1_prompt = format!("{}, enter your guess:", p1);
            let p1_guess = prompt_no_show(&p1_prompt)?;
            word_game.submit_guess(&p1.username, &p1_guess)?;
            
            let p2_prompt = format!("{}, enter your guess:", p2);
            let p2_guess = prompt_no_show(&p2_prompt)?;
            word_game.submit_guess(&p2.username, &p2_guess)?;

            if !word_game.is_game_complete(&game_id)? {
                println!("Aww, shucks... Those didn't match.");
                let game = word_game.get_game(&game_id)?;
                println!("{}", &game);
            }
        }

        println!("{} and {}, you won!!! Congrats!", p1, p2);
        let game = word_game.get_game(&game_id)?;
        println!("{}", &game);

        Ok(())
    }
}

// fn print_guesses(word_game: &WordGame, game_id: &str) -> AppResult<()> {
//     for (i, (g1, g2)) in word_game.get_guesses(game_id)?.iter().enumerate() {
//         let round = i + 1;
//         println!("{}) {}\t{}", round, g1.as_ref().unwrap(), g2.as_ref().unwrap())
//     }

//     Ok(())
// }

fn prompt(prompt: &str) -> String {
    print!("{} ", prompt);
    stdout()
        .flush()
        .expect("The console seems clogged. I couldn't flush it...");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("I don't understand what you're saying...");
    input.trim().into()
}

fn prompt_no_show(prompt: &str) -> AppResult<String> {
    let input = prompt_password_stdout(&format!("{} ", prompt))
        .map_err(|e| ApplicationError::new(&format!("{:?}", e.kind()), "Could not read guess", None))?;

    Ok(input)
}