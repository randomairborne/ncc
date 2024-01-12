use std::{fmt::Display, time::Duration};

use rand::distributions::{Alphanumeric, DistString};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    select,
};

use crate::{
    board::{Board, Color},
    Error, State,
};

pub trait TermUtils {
    async fn write_str(&mut self, item: &str) -> Result<(), Error>;
    async fn read_line(&mut self) -> Result<String, Error>;
    // async fn clear(&mut self) -> Result<(), Error> {
    //     const CLEAR: &str = "\u{1b}[2J\n";
    //     self.write_str(CLEAR).await
    // }
}

impl TermUtils for TcpStream {
    async fn write_str(&mut self, item: &str) -> Result<(), Error> {
        self.write_all(item.to_string().as_bytes()).await?;
        Ok(())
    }

    async fn read_line(&mut self) -> Result<String, Error> {
        let mut text_bytes = Vec::new();
        loop {
            let item = self.read_u8().await?;
            if item as char == '\n' {
                break;
            }
            text_bytes.push(item);
        }
        Ok(String::from_utf8_lossy(&text_bytes).into())
    }
}

impl State {
    pub async fn handle(self, stream: TcpStream) {
        if let Err(e) = self.connect_or_create(stream).await {
            eprintln!("Error: {e:?}");
        }
    }

    #[async_recursion::async_recursion]
    async fn connect_or_create(&self, mut stream: TcpStream) -> Result<(), Error> {
        stream.write_str("Join or create room? [j/c]\n").await?;
        let string = stream.read_line().await?;
        if string.starts_with('j') {
            self.join_game(stream).await?;
        } else if string.starts_with('c') {
            self.new_game(stream).await?;
        } else {
            stream
                .write_str("Please write `join`/`j` or `create`/`c`.\n")
                .await?;
        }
        Ok(())
    }

    async fn join_game(&self, mut stream: TcpStream) -> Result<(), Error> {
        stream.write_str("input game code\n").await?;
        let code_input = stream.read_line().await?;
        let code = code_input.trim();
        let Some(sender) = self.pending.write().remove(code) else {
            stream.write_str("\nunknown game code\n").await?;
            return Ok(());
        };
        if let Err(mut stream) = sender.send(stream) {
            stream.write_str("\nCould not join game\n").await?;
        }
        Ok(())
    }

    async fn new_game(&self, mut white: TcpStream) -> Result<(), Error> {
        let code = Alphanumeric
            .sample_string(&mut rand::thread_rng(), 6)
            .to_ascii_uppercase();
        let (stream_tx, mut stream_rx) = tokio::sync::oneshot::channel();
        self.pending.write().insert(code.clone(), stream_tx);
        let remind_dur = Duration::from_secs(1);
        let mut dots = 1;
        white.write_str(&format!("Code: {code}\n")).await?;
        let mut black: TcpStream = loop {
            let message = format!(
                "\rWaiting for opponent{}{}",
                ".".repeat(dots),
                " ".repeat(3 - dots)
            );
            white.write_str(&message).await?;
            if dots >= 3 {
                dots = 1;
            } else {
                dots += 1;
            }
            select! {
                black = &mut stream_rx => break black?,
                _ = tokio::time::sleep(remind_dur) => {}
            }
        };
        if let Err(err) = self.run_game(&mut white, &mut black).await {
            let disconnect_msg = format!("\nDisconnected: {err}\n");
            let _ = white.write_str(&disconnect_msg).await;
            let _ = black.write_str(&disconnect_msg).await;
        }
        Ok(())
    }

    async fn run_game(&self, white: &mut TcpStream, black: &mut TcpStream) -> Result<(), Error> {
        let mut board = Board::new();
        let mut turn = Color::White;
        loop {
            white.write_str(&board.display(Color::White)?).await?;
            black.write_str(&board.display(Color::Black)?).await?;
            turn = match turn {
                Color::White => self.turn(&mut board, turn, white).await,
                Color::Black => self.turn(&mut board, turn, black).await,
            }?;
        }
    }

    async fn turn(
        &self,
        board: &mut Board,
        turn: Color,
        active: &mut TcpStream,
    ) -> Result<Color, Error> {
        active.write_str("Your turn!\n").await?;
        loop {
            let notation = active.read_line().await?;
            match board.turn(notation) {
                Ok(()) => return Ok(turn.flipped()),
                Err(e) => active.write_str(&format!("\n{e}\n")).await?,
            }
        }
    }
}
