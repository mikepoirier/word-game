use std::{
    convert::TryFrom,
    sync::{Arc, Mutex}, str::FromStr,
};

use async_trait::async_trait;

use futures::StreamExt;
use tokio_xmpp::{AsyncClient, Event};
use xmpp_parsers::{
    disco::{DiscoInfoResult, Feature},
    iq::{Iq, IqType},
    message::{Body, Message, MessageType},
    ns::{DISCO_INFO, RECEIPTS},
    presence::{Presence, Show as PresenceShow, Type as PresenceType},
    receipts::Received,
    BareJid, Element, Jid,
};

use crate::{
    game::{player::PlayerStatus, WordGame},
    AppResult, ApplicationError,
};

use super::Runner;

pub struct XmppRunner {
    jid: String,
    password: String,
    running: bool,
}

impl XmppRunner {
    pub fn new() -> Self {
        let jid = std::env::var("WORD_GAME_XMPP_JID").expect("Expected WORD_GAME_XMPP_JID");
        let password =
            std::env::var("WORD_GAME_XMPP_PASSWORD").expect("Expected WORD_GAME_XMPP_PASSWORD");
        Self {
            jid,
            password,
            running: true,
        }
    }
}

impl Default for XmppRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(?Send)]
impl Runner for XmppRunner {
    async fn run(&mut self, game: Arc<Mutex<WordGame>>) -> AppResult<()> {
        let mut client = AsyncClient::new(&self.jid, &self.password).unwrap();

        client.set_reconnect(false);

        while self.running {
            if let Some(event) = client.next().await {
                match event {
                    Event::Online { bound_jid, .. } => {
                        handle_online(&bound_jid, &mut client).await;
                    }
                    Event::Stanza(s) => {
                        handle_stanza(s, &mut client, game.clone()).await;
                    }
                    _ => {}
                }
            }
        }

        client
            .send_end()
            .await
            .map_err(|e| ApplicationError::new("Close Client Error", &format!("{}", e), None))?;

        Ok(())
    }
}

async fn handle_online(bound_jid: &Jid, client: &mut AsyncClient) {
    println!("Online at {}", bound_jid);
    let presence = make_presence();
    client.send_stanza(presence).await.unwrap();
}

async fn handle_stanza(stanza: Element, client: &mut AsyncClient, game: Arc<Mutex<WordGame>>) {
    if let Some(presence) = Presence::try_from(stanza.clone()).ok() {
        match (&presence.from, &presence.type_) {
            (Some(ref from), PresenceType::Subscribe) => {
                println!("Got subscribe presence from: {}", &from);
                let subscribed = allow_presence_subscribe(from.clone());
                client.send_stanza(subscribed).await.unwrap();
            }
            (Some(ref from), PresenceType::Subscribed) => {
                println!("Got subscribed presence from: {}", &from);
            }
            _ => {}
        }
    } else if let Some(message) = Message::try_from(stanza.clone()).ok() {
        match (
            message.id,
            message.from,
            message.bodies.get(""),
            message.payloads,
        ) {
            (Some(id), Some(from), Some(body), payloads) if message.type_ != MessageType::Error => {
                handle_ack(payloads, &from, id, client).await;

                let username = format!("{}", BareJid::from(from.clone()));
                let mut game = game.lock().unwrap();
                let player = game.get_player(&username).unwrap();

                match player.status {
                    PlayerStatus::New => {
                        let message = "Hello there! We haven't met before. What is your name?";
                        let reply = make_reply(from.clone(), message);
                        client.send_stanza(reply).await.unwrap();
                        game.change_player_to_introducing(&username).unwrap();
                    }
                    PlayerStatus::Introducing => {
                        let display_name = &body.0;
                        let player = game.introduce_player(&username, display_name).unwrap();
                        let message = format!("Nice to meet you {}! Welcome to the word game! If you would like to create a game, use the `/createGame` command. If you would like to join a game, use the `/join` command.", player);
                        let reply = make_reply(from.clone(), &message);
                        client.send_stanza(reply).await.unwrap();
                    }
                    PlayerStatus::InLobby => {
                        let player_message = &body.0;
                        if player_message.starts_with("/createGame") {
                            let game_id = game.new_game(&username).unwrap();
                            let message = format!("You have just created a new game! Give the code `{}` to your friend that you want to play with!", game_id);
                            let reply = make_reply(from.clone(), &message);
                            client.send_stanza(reply).await.unwrap();
                        } else if player_message.starts_with("/join") {
                            game.debug();
                            match player_message.split_ascii_whitespace().nth(1) {
                                Some(game_id) => {
                                    match game.join_game(&username, game_id) {
                                        Ok(_) => {
                                            let message = "You have joined the game!";
                                            let reply = make_reply(from.clone(), message);
                                            client.send_stanza(reply).await.unwrap();
                                            let other_players = game.get_players_in_game(game_id).unwrap();
                                            for username in other_players {
                                                if username == player.username {
                                                    break;
                                                }
                                                let jid = Jid::from_str(&username).unwrap();
                                                let message = format!("{} has joined the game!", player);
                                                let reply = make_reply(jid, &message);
                                                client.send_stanza(reply).await.unwrap();
                                            }
                                        }
                                        Err(e) => {
                                            let message = format!("There was an issue joining the game. Here are the details: {}", e.message);
                                            let reply = make_reply(from.clone(), &message);
                                            client.send_stanza(reply).await.unwrap();
                                        }
                                    }
                                }
                                None => {
                                    let message = "Please include a game ID!";
                                    let reply = make_reply(from.clone(), message);
                                    client.send_stanza(reply).await.unwrap();
                                }
                            }
                        }
                    }
                    PlayerStatus::InGame { ref game_id } => {
                        let player_message = &body.0;
                        if player_message.starts_with("/guess") {
                            match player_message.split_ascii_whitespace().nth(1) {
                                Some(guess) => {
                                    match game.submit_guess(&username, game_id, guess) {
                                        Ok(win) => {
                                            if win {
                                                let players = game.get_players_in_game(game_id).unwrap();
                                                for username in players {
                                                    let jid = Jid::from_str(&username).unwrap();
                                                    let message = "You won!!!";
                                                    let reply = make_reply(jid, message);
                                                    client.send_stanza(reply).await.unwrap();
                                                }
                                            }
                                        },
                                        Err(_) => {

                                        },
                                    }
                                },
                                None => {
                                    let message = "Please include a guess!";
                                    let reply = make_reply(from.clone(), message);
                                    client.send_stanza(reply).await.unwrap();
                                },
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    } else if let Some(iq) = Iq::try_from(stanza.clone()).ok() {
        match (&iq.from, &iq.payload, &iq.id) {
            (Some(ref from), IqType::Get(element), id) => {
                println!("IQ from {}: {:?}", from, element);
                if element.has_ns(DISCO_INFO) {
                    let response = make_service_discovery(&from, id);
                    client.send_stanza(response).await.unwrap();
                }
            }
            _ => {
                println!("Unhandled Iq: {:?}", iq);
            }
        }
    } else {
        println!("Unhandled stanza: {:?}", stanza);
    }
}

async fn handle_ack(payloads: Vec<Element>, from: &Jid, id: String, client: &mut AsyncClient) {
    if should_ack(payloads) {
        let receipt = make_receipt(from.clone(), &id);
        client.send_stanza(receipt).await.unwrap();
    }
}

fn make_presence() -> Element {
    let mut presence = Presence::new(PresenceType::None);
    presence.show = Some(PresenceShow::Chat);
    presence
        .statuses
        .insert(String::from("en"), String::from("Echoing messages."));
    presence.into()
}

fn allow_presence_subscribe(to: Jid) -> Element {
    let presence = Presence::new(PresenceType::Subscribed)
        .with_show(PresenceShow::Chat)
        .with_to(to);
    presence.into()
}

fn make_reply(to: Jid, body: &str) -> Element {
    let mut message = Message::new(Some(to));
    message.bodies.insert(String::new(), Body(body.to_owned()));
    message.into()
}

fn should_ack(payloads: Vec<Element>) -> bool {
    payloads
        .iter()
        .any(|element| element.name() == "request" && element.ns() == "urn:xmpp:receipts")
}

fn make_receipt(to: Jid, id: &str) -> Element {
    let mut message = Message::new(Some(to));
    let receipt = Received { id: id.to_owned() };
    message.payloads.push(receipt.into());

    message.into()
}

fn make_service_discovery(to: &Jid, id: &str) -> Element {
    let disco_info = DiscoInfoResult {
        node: None,
        identities: vec![],
        features: vec![Feature::new(RECEIPTS), Feature::new(DISCO_INFO)],
        extensions: vec![],
    };
    let iq = Iq::from_result(id, Some(disco_info)).with_to(to.clone());

    iq.into()
}
