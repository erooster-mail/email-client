use imap::Session;
use relm4::SharedState;
use rustls_connector::TlsStream;
use std::net::TcpStream;

pub enum ImapSession {
    TLS(Session<TlsStream<TcpStream>>),
    TCP(Session<TcpStream>),
    None,
}

impl Default for ImapSession {
    fn default() -> Self {
        Self::None
    }
}

pub static IMAP_SESSION: SharedState<ImapSession> = SharedState::new();
