use std::{fmt, sync::{Mutex, Arc}};
use async_trait::async_trait;

use crate::{game::WordGame, AppResult};

use self::{xmpp::XmppRunner, console::ConsoleRunner};

pub mod xmpp;
pub mod console;

#[async_trait(?Send)]
pub trait Runner {
    async fn run(&mut self, game: Arc<Mutex<WordGame>>) -> AppResult<()>;
}

#[derive(Debug, Clone)]
pub struct RunnerError {
    detail: String,
}

impl fmt::Display for RunnerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Runner Error: {}", self.detail)
    }
}

pub enum RunnerType {
    Console,
    XMPP,
}

pub struct RunnerFactory;

impl RunnerFactory {
    pub fn create(runner_type: RunnerType) -> Box<dyn Runner>
    {
        match runner_type {
            RunnerType::XMPP => {
                Box::new(XmppRunner::new())
            },
            RunnerType::Console => Box::new(ConsoleRunner::new()),
        }
    }
}