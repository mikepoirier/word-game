use std::sync::{Arc, Mutex};

use word_game::{
    database::{DatabaseFactory, DatabaseType},
    game::WordGame,
    runner::{RunnerFactory, RunnerType},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database = DatabaseFactory::create(DatabaseType::InMemory);
    let database = Arc::new(Mutex::new(database));
    let game = WordGame::new(database);
    let runner = RunnerFactory::create(RunnerType::Console);
    let game = Arc::new(Mutex::new(game));

    match runner.run(game).await {
        Ok(_r) => {
            println!("Hope you had fun!");
        }
        Err(e) => {
            println!("Oh no! There was an error: {:?}", e);
        }
    };

    Ok(())
}
