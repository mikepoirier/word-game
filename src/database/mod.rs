use crate::{game::{Player, Game}, AppResult, trait_enum};

use self::in_memory::InMemoryDatabase;

pub mod in_memory;

pub enum DatabaseType {
    InMemory
}

// Try implementing this to remove need to use Box<dyn Database>:
// https://singpolyma.net/2018/09/rust-factory-without-box-trait-object/
pub trait Database {
    fn save_player(&mut self, player: Player) -> AppResult<()>;
    fn get_players(&self) -> AppResult<Vec<Player>>;
    fn get_player_by_username(&self, username: &str) -> AppResult<Option<Player>>;
    fn save_game(&mut self, game: Game) -> AppResult<()>;
    fn get_games(&self) -> AppResult<Vec<Game>>;
    fn get_game(&self, game_id: &str) -> AppResult<Option<Game>>;
}

trait_enum!(Database, DatabaseEnum, InMemoryDatabase);

pub struct DatabaseFactory;

impl DatabaseFactory {
    pub fn create(database_type: DatabaseType) -> DatabaseEnum {
        match database_type {
            DatabaseType::InMemory => DatabaseEnum::InMemoryDatabase(InMemoryDatabase::new())
        }
    }
}