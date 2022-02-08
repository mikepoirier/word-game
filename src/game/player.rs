#[derive(Debug, Clone)]
pub struct Player {
    pub username: String,
    pub display_name: Option<String>,
    pub status: PlayerStatus,
}

impl Player {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.into(),
            display_name: None,
            status: PlayerStatus::New,
        }
    }

    pub fn set_display_name(&mut self, display_name: &str) {
        self.display_name = Some(display_name.to_string());
    }

    pub fn set_status(&mut self, status: PlayerStatus) {
        self.status = status;
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.display_name {
            Some(name) => write!(f, "{}", name),
            None => write!(f, "{}", self.username),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PlayerStatus {
    New,
    Introducing,
    InLobby,
    InGame { game_id: String }
}

impl std::fmt::Display for PlayerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerStatus::New => write!(f, "NEW"),
            PlayerStatus::Introducing => write!(f, "INTRODUCING"),
            PlayerStatus::InLobby => write!(f, "IN_LOBBY"),
            PlayerStatus::InGame { game_id } => write!(f, "IN_GAME({})", game_id),
        }
    }
}
