use std::{sync::{Arc, Mutex}, convert::TryFrom};

use async_trait::async_trait;

use tokio_xmpp::{Event};
use xmpp_parsers::{
    message::{Message, Body}, 
    presence::{Presence, Show as PresenceShow, Type as PresenceType},
    Element, Jid
};

use crate::{game::WordGame, AppResult};

use super::Runner;

pub struct XmppRunner {
    // client: AsyncClient,
    running: bool,
}

impl XmppRunner {
    pub fn new() -> Self {
        // let client = AsyncClient::new("", "").unwrap();
        Self {
            // client,
            running: true,
        }
    }
}

impl Default for XmppRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Runner for XmppRunner {
    async fn run(&self, _game: Arc<Mutex<WordGame>>) -> AppResult<()> {
        // while self.running {
        //     let client = self.client.clone();
        //     let client = client.lock().unwrap();
        //     if let Some(event) = client.next().await {
        //         if let Some(message) = parse_message(event) {
        //             match (message.from, message.bodies.get("")) {
        //                 (Some(ref from), Some(body)) if body.0 == "die" => {
        //                     break;
        //                 }
        //                 (Some(ref from), Some(body)) => {
        //                     let reply = make_reply(from.clone(), "Hello! I am the Word Game Bot!");
        //                     client.send_stanza(reply).await.unwrap();
        //                 }
        //                 _ => {}
        //             }
        //         }
        //     }
        // };

        Ok(())
    }
}

fn parse_message(event: Event) -> Option<Message> {
    event
        .into_stanza()
        .and_then(|stanza| Message::try_from(stanza).ok())
}

fn make_presence() -> Element {
    let mut presence = Presence::new(PresenceType::None);
    presence.show = Some(PresenceShow::Chat);
    presence
        .statuses
        .insert(String::from("en"), String::from("Echoing messages."));
    presence.into()
}

fn make_reply(to: Jid, body: &str) -> Element {
    let mut message = Message::new(Some(to));
    message.bodies.insert(String::new(), Body(body.to_owned()));
    message.into()
}
