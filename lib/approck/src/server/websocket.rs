use std::borrow::Cow;

use granite::ResultExt;
use salvo_core::{Request, Response};
use salvo_extra::websocket::{
    Message as SalvoMessage, WebSocket as SalvoWebsocket, WebSocketUpgrade as SalvoWebSocketUpgrade,
};

pub struct Message(SalvoMessage);

impl From<&str> for Message {
    fn from(value: &str) -> Self {
        value.to_owned().into()
    }
}

impl From<String> for Message {
    fn from(value: String) -> Self {
        Self::new(MessageData::Text(value))
    }
}

impl From<Vec<u8>> for Message {
    fn from(value: Vec<u8>) -> Self {
        Self::new(MessageData::Binary(value))
    }
}

impl From<&[u8]> for Message {
    fn from(value: &[u8]) -> Self {
        value.to_owned().into()
    }
}

pub enum MessageDataRef<'a> {
    Text(&'a str),
    Binary(&'a [u8]),
    Ping(&'a [u8]),
    Pong(&'a [u8]),
    Close,
}

pub enum MessageData {
    Text(String),
    Binary(Vec<u8>),
    Ping(Vec<u8>),
    Pong(Vec<u8>),
    Close,
}

impl Message {
    pub fn new(data: MessageData) -> Self {
        Self(match data {
            MessageData::Text(data) => SalvoMessage::text(data),
            MessageData::Binary(data) => SalvoMessage::binary(data),
            MessageData::Ping(data) => SalvoMessage::ping(data),
            MessageData::Pong(data) => SalvoMessage::pong(data),
            MessageData::Close => todo!(),
        })
    }

    pub fn close_with<CODE: Into<u16>, REASON: Into<Cow<'static, str>>>(
        code: CODE,
        reason: REASON,
    ) -> Message {
        Message(SalvoMessage::close_with(code, reason))
    }

    pub fn data(&self) -> MessageDataRef<'_> {
        if self.0.is_text() {
            return MessageDataRef::Text(
                self.0.to_str().expect("Expected this to be a text message"),
            );
        }
        if self.0.is_binary() {
            return MessageDataRef::Binary(self.0.as_bytes());
        }
        if self.0.is_close() {
            return MessageDataRef::Close;
        }
        if self.0.is_ping() {
            return MessageDataRef::Ping(self.0.as_bytes());
        }
        if self.0.is_pong() {
            return MessageDataRef::Pong(self.0.as_bytes());
        }
        unreachable!()
    }

    pub fn into_data(self) -> MessageData {
        if self.0.is_text() {
            return MessageData::Text(
                String::from_utf8(self.0.into_bytes()).expect("Expected valid text message"),
            );
        }
        if self.0.is_binary() {
            return MessageData::Binary(self.0.into_bytes());
        }
        if self.0.is_close() {
            return MessageData::Close;
        }
        if self.0.is_ping() {
            return MessageData::Ping(self.0.into_bytes());
        }
        if self.0.is_pong() {
            return MessageData::Pong(self.0.into_bytes());
        }
        unreachable!()
    }
}

/// A websocket connection
pub struct WebSocket(SalvoWebsocket);

impl WebSocket {
    pub async fn send(&mut self, message: Message) -> granite::Result<()> {
        self.0.send(message.0).await.amend(|mut e| {
            e.kind = granite::ErrorKind::WebsocketCommunication;
            e
        })
    }

    pub async fn recv(&mut self) -> Option<granite::Result<Message>> {
        self.0.recv().await.map(|r| {
            r.map(Message).amend(|mut e| {
                e.kind = granite::ErrorKind::WebsocketCommunication;
                e
            })
        })
    }

    pub async fn close(self) -> granite::Result<()> {
        self.0.close().await.amend(|mut e| {
            e.kind = granite::ErrorKind::WebsocketCommunication;
            e
        })
    }
}

pub async fn upgrade<H, F>(
    req: &mut Request,
    handler: H,
) -> granite::Result<crate::server::response::WebSocketUpgrade>
where
    H: Fn(WebSocket) -> F + Send + 'static,
    F: std::future::Future<Output = ()> + Send + 'static,
{
    let mut response = Response::new();
    SalvoWebSocketUpgrade::new()
        .upgrade(req, &mut response, |ws| async move {
            handler(WebSocket(ws)).await;
        })
        .await
        .amend(|mut e| {
            e.kind = granite::ErrorKind::WebsocketUpgrade;
            e
        })?;

    Ok(crate::server::response::WebSocketUpgrade(response))
}
