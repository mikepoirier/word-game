use std::{sync::Mutex, collections::HashMap};

use crate::domain::player::Player;

pub trait PlayerRepository {
    fn save(&self, player: Player) -> Player;
    fn get(&self, id: &str) -> Option<Player>;
    fn delete(&self, id: &str) -> Option<Player>;
}

pub struct InMemoryPlayerRepository {
    data: Mutex<HashMap<String, Player>>,
}

impl InMemoryPlayerRepository {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
        }
    }
}

impl PlayerRepository for InMemoryPlayerRepository {
    fn save(&self, player: Player) -> Player {
        let mut data = self.data.lock().unwrap();
        data.insert(player.id(), player.clone());
        player
    }

    fn get(&self, id: &str) -> Option<Player> {
        let data = self.data.lock().unwrap();
        if data.contains_key(id) {
            Some(data[id].clone())
        } else {
            None
        }
    }

    fn delete(&self, id: &str) -> Option<Player> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_player_on_save() {
        let player = Player::new("id", "username");
        let expected = player.clone();
        let repo = InMemoryPlayerRepository::new();

        let actual = repo.save(player);

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_can_get_a_player_by_id() {
        let id = "id-123";
        let player = Player::new(id, "username-123");
        let expected = Some(player.clone());
        let repo = InMemoryPlayerRepository::new();
        repo.save(player);

        let actual = repo.get(id);

        assert_eq!(actual, expected);
    }
}
