use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    XMPP {
        #[clap(short, long, env = "WORD_GAME_XMPP_USERNAME")]
        username: String,
        #[clap(short, long, env = "WORD_GAME_XMPP_PASSWORD")]
        password: String,
    },
    Console,
}
