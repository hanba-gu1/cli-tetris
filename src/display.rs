use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::{
    io::{stdout, Result},
    sync::{Arc, Mutex},
};

use crate::GameState;

mod field;
mod slot;

enum DisplayMessage {
    All,
    Field,
    Hold,
    Next,
    Exit,
}

pub struct Displayer {
    handle: tokio::task::JoinHandle<Result<()>>,
    sender: std::sync::mpsc::Sender<DisplayMessage>,
}
impl Displayer {
    pub fn new(game_state: Arc<Mutex<GameState>>) -> Result<Self> {
        let (sender, receiver) = std::sync::mpsc::channel();
        let handle = tokio::task::spawn_blocking(move || -> Result<()> {
            let hold_slot_column = 0;
            let hold_slot_row = 0;
            let field_column = 15;
            let field_row = 0;
            let next_slot_column = 40;
            let next_slot_row = 0;

            while let Ok(display_message) = receiver.recv() {
                let game_state = game_state.lock().unwrap();

                let next_minos = game_state.next_minos.as_slices();
                let next_minos = [next_minos.0, next_minos.1].concat();
                match display_message {
                    DisplayMessage::All => {
                        execute!(stdout(), Clear(ClearType::All))?;
                        slot::display_hold(hold_slot_column, hold_slot_row, &game_state.held_mino)?;
                        slot::display_next(next_slot_column, next_slot_row, &next_minos)?;
                        field::display_field(field_column, field_row, &game_state)?;
                    }
                    DisplayMessage::Field => {
                        field::display_field(field_column, field_row, &game_state)?
                    }
                    DisplayMessage::Hold => {
                        slot::display_hold(hold_slot_column, hold_slot_row, &game_state.held_mino)?
                    }
                    DisplayMessage::Next => {
                        slot::display_next(next_slot_column, next_slot_row, &next_minos)?
                    }
                    DisplayMessage::Exit => break,
                }
            }

            Ok(())
        });
        Ok(Self { handle, sender })
    }

    pub fn all(&self) {
        let _ = self.sender.send(DisplayMessage::All);
    }
    pub fn field(&self) {
        let _ = self.sender.send(DisplayMessage::Field);
    }
    pub fn hold(&self) {
        let _ = self.sender.send(DisplayMessage::Hold);
    }
    pub fn next(&self) {
        let _ = self.sender.send(DisplayMessage::Next);
    }
    pub async fn exit(self) -> Result<()> {
        let _ = self.sender.send(DisplayMessage::Exit);
        self.handle.await?
    }
}
