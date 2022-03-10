use args::Args;
use error::AppError;

use crate::xmpp::XmppRunner;

pub mod args;
pub mod domain;
pub mod error;
pub mod repository;
pub mod xmpp;

pub type AppResult<T> = Result<T, AppError>;

pub async fn run(args: &Args) -> AppResult<()> {
    println!("Hello, World!");
    println!("Args: {:?}", args);

    if let args::Commands::XMPP { username, password } = &args.command {
        let mut xmpp = XmppRunner::new(username, password)?;
        xmpp.run().await?;
    }

    Ok(())
}
