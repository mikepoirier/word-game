
#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    id: String,
    username: String,
}

impl Player {
    pub fn new(id: &str, username: &str) -> Self {
        Self {
            id: id.to_owned(),
            username: username.to_owned()
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }
}