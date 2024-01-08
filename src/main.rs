use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use parking_lot::RwLock;
use thiserror::Error;
use tokio::{
    net::{TcpListener, TcpStream},
    select,
    sync::oneshot::Sender as OneshotSender,
    task::JoinSet,
};

use crate::board::Board;

mod board;
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
    let state = State {
        pending: Arc::new(RwLock::new(HashMap::new())),
    };
    println!("Listening on {addr}");
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
        connections.spawn(state.clone().handle(stream));
    }
    while connections.join_next().await.is_some() {}
    println!("All games done, bye!");
}

#[derive(Clone)]
pub struct State {
    pending: Arc<RwLock<HashMap<String, OneshotSender<TcpStream>>>>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Chess notation was too short")]
    BadNotationLength,
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Format error")]
    Fmt(#[from] std::fmt::Error),
    #[error("Room join error")]
    OneshotRecv(#[from] tokio::sync::oneshot::error::RecvError),
}
