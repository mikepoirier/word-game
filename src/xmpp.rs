use std::convert::TryFrom;

use async_trait::async_trait;
use futures::StreamExt;
use tokio_xmpp::{AsyncClient, Event};
use xmpp_parsers::{presence::Presence, message::Message};

use crate::AppResult;

#[async_trait(?Send)]
trait XmppEventHandler {
    async fn handle(&self, event: &Event, client: &mut AsyncClient) -> AppResult<()>;
}

pub struct XmppRunner {
    client: AsyncClient,
    handlers: Vec<Box<dyn XmppEventHandler>>,
}

impl XmppRunner {
    pub fn new(username: &str, password: &str) -> AppResult<Self> {

        let mut client = AsyncClient::new(username, password)?;
        client.set_reconnect(false);

        let handlers: Vec<Box<dyn XmppEventHandler>> = vec![
            Box::new(SendOnlinePresenceHandler::default()),
            Box::new(MessageLogger::default())
        ];


        Ok(Self { client, handlers })
    }

    pub async fn run(&mut self) -> AppResult<()> {

        while let Some(event) = self.client.next().await {
            self.handle_event(&event).await?;
        }

        self.client.send_end().await?;

        Ok(())
    }

    async fn handle_event(&mut self, event: &Event) -> AppResult<()> {
        for handler in &self.handlers {
            handler.handle(event, &mut self.client).await?;
        }

        Ok(())
    }
}

struct SendOnlinePresenceHandler;

impl Default for SendOnlinePresenceHandler {
    fn default() -> Self {
        Self { }
    }
}

#[async_trait(?Send)]
impl XmppEventHandler for SendOnlinePresenceHandler {
    async fn handle(&self, event: &Event, client: &mut AsyncClient) -> AppResult<()> {
        if let tokio_xmpp::Event::Online { bound_jid, .. } = event {
            println!("Online at: {}", bound_jid);
            let mut presence = Presence::new(xmpp_parsers::presence::Type::None)
                .with_show(xmpp_parsers::presence::Show::Chat);
            presence.set_status("en", "Just playing games!");
            
            client.send_stanza(presence.into()).await?;
        }
        
        Ok(())
    }
}

struct MessageLogger;

impl Default for MessageLogger {
    fn default() -> Self {
        Self {}
    }
}

#[async_trait(?Send)]
impl XmppEventHandler for MessageLogger {
    async fn handle(&self, event: &Event, _client: &mut AsyncClient) -> AppResult<()> {
        if let tokio_xmpp::Event::Stanza(element) = event {
            if let Some(message) = Message::try_from(element.clone()).ok() {
                let body = message.bodies.get("");
                let from = &message.from;

                if let (Some(body), Some(from)) = (body, from) {
                    println!("Message from {}: {:?}", from, body.0);
                }
            }
        }
        
        Ok(())
    }
}
