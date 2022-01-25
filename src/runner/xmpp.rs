use std::{
    convert::TryFrom,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;

use futures::StreamExt;
use tokio_xmpp::{AsyncClient, Event};
use xmpp_parsers::{
    disco::{DiscoInfoResult, Feature},
    iq::{Iq, IqType},
    message::{Body, Message, MessageType},
    presence::{Presence, Show as PresenceShow, Type as PresenceType},
    receipts::Received,
    Element, Jid, ns::{DISCO_INFO, RECEIPTS},
};

use crate::{game::WordGame, AppResult};

use super::Runner;

pub struct XmppRunner {
    jid: String,
    password: String,
    running: bool,
}

impl XmppRunner {
    pub fn new() -> Self {
        Self {
            jid: String::from("echobot@chat.poirier.cloud"),
            password: String::from("3choBot!"),
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
    async fn run(&mut self, _game: Arc<Mutex<WordGame>>) -> AppResult<()> {
        let mut client = AsyncClient::new(&self.jid, &self.password).unwrap();

        client.set_reconnect(false);

        while self.running {
            if let Some(event) = client.next().await {
                match event {
                    Event::Online { bound_jid, .. } => {
                        handle_online(&bound_jid, &mut client).await;
                    }
                    Event::Stanza(s) => {
                        handle_stanza(s, &mut client).await;
                    }
                    _ => {}
                }
                // println!(">>> Event in {:?}", event);
                // if event.is_online() {
                //     send_online_presence(&mut client, event).await;
                // } else if event.is_stanza("presence") {
                //     let presence = parse_presence(event).unwrap();
                //     match (presence.from, presence.type_) {
                //         (Some(ref from), PresenceType::Subscribe) => {
                //             println!("Got subscribe presence from: {}", &from);
                //             let subscribed = allow_presence_subscribe(from.clone());
                //             client.send_stanza(subscribed).await.unwrap();
                //         }
                //         (Some(ref from), PresenceType::Subscribed) => {
                //             println!("Got subscribed presence from: {}", &from);
                //         }
                //         _ => {}
                //     }
                // } else if let Some(message) = parse_message(event) {
                //     match (message.from, message.bodies.get("")) {
                //         (Some(ref _from), Some(body)) if body.0 == "die" => {
                //             self.running = false;
                //         }
                //         (Some(ref from), Some(_body)) => {
                //             if message.type_ != MessageType::Error {
                //                 // let presence_accept = allow_presence_subscribe(from.clone());
                //                 // client.send_stanza(presence_accept).await.unwrap();
                //                 let reply = make_reply(from.clone(), "Hello!");
                //                 client.send_stanza(reply).await.unwrap();
                //             }
                //         }
                //         _ => {}
                //     }
                // }
            }
        }

        Ok(())
    }
}

async fn handle_online(bound_jid: &Jid, client: &mut AsyncClient) {
    println!("Online at {}", bound_jid);
    let presence = make_presence();
    client.send_stanza(presence).await.unwrap();
}

async fn handle_stanza(stanza: Element, client: &mut AsyncClient) {
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
            (Some(id), Some(ref from), Some(_body), payloads)
                if message.type_ != MessageType::Error =>
            {
                if should_ack(payloads) {
                    println!("Ack requested for message {}", id);
                    let receipt = make_receipt(from.clone(), &id);
                    client.send_stanza(receipt).await.unwrap();
                }

                let reply = make_reply(from.clone(), "Hello!");
                client.send_stanza(reply).await.unwrap();
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
    payloads.iter().any(|element| {
        element.name() == "request" && element.ns() == "urn:xmpp:receipts"
    })
}

fn make_receipt(to: Jid, id: &str) -> Element {
    let mut message = Message::new(Some(to));
    let receipt = Received { id: id.to_owned() };
    message.payloads.push(receipt.into());

    message.into()
}

fn make_service_discovery(to: &Jid, id: &str) -> Element {
    let disco_info = DiscoInfoResult{
        node: None,
        identities: vec![],
        features: vec![Feature::new(RECEIPTS)],
        extensions: vec![],
    };
    let iq = Iq::from_result(id, Some(disco_info))
        .with_to(to.clone());

    iq.into()
}
