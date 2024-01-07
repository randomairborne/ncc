use crate::board::{Board, Color};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::select;
use tokio::sync::{
    mpsc::{Receiver as MpscReceiver, Sender as MpscSender},
    oneshot::Receiver as OneshotReceiver,
};
use tokio::task::JoinSet;

mod board;
mod channel_server;
mod net;
mod turn;

#[tokio::main]
async fn main() {
    let board = Board::new();
    let addr = SocketAddr::from(([0, 0, 0, 0], 30484));
    let listener = TcpListener::bind(addr).await.unwrap();
    let (shutdown_tx, mut shutdown) = tokio::sync::oneshot::channel();
    tokio::task::spawn(async move {
        vss::shutdown_signal().await;
        let _ = shutdown_tx.send(());
    });
    let mut connections = JoinSet::new();
    let state = State;
    loop {
        let socket = select! {
            value = listener.accept() => value,
            _ = &mut shutdown => {
                break;
            }
        };
        let (stream, _socket_addr) = match socket {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{e:?}");
                continue;
            }
        };
        connections.spawn(net::handle(state.clone(), stream));
    }
    while connections.join_next().await.is_some() {}
}

#[derive(Clone)]
pub struct State {
    pending: Arc<RwLock<HashMap<String, (MpscSender<ChessMessage>, MpscReceiver<ChessMessage>)>>>,
}

pub enum ChessMessage {}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Chess notation was too short")]
    BadNotationLength,
}
