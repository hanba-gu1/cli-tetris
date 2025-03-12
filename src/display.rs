use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::{
    io::{stdout, Result},
    sync::{Arc, Mutex},
};

use crate::{field, slot, GameState};

pub struct Displayer {
    sender: std::sync::mpsc::Sender<()>,
    handle: tokio::task::JoinHandle<Result<()>>,
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

            while let Ok(_) = receiver.recv() {
                if receiver.try_recv().is_ok() {
                    continue;
                }

                let game_state = game_state.lock().unwrap();

                let next_minos = game_state.next_minos.as_slices();
                let next_minos = [next_minos.0, next_minos.1].concat();
                execute!(stdout(), Clear(ClearType::All))?;
                slot::display_hold(hold_slot_column, hold_slot_row, &game_state.held_mino)?;
                slot::display_next(next_slot_column, next_slot_row, &next_minos)?;
                field::display_field(
                    field_column,
                    field_row,
                    &game_state.field,
                    &game_state.current_mino,
                )?;
            }

            Ok(())
        });
        Ok(Self { sender, handle })
    }

    pub fn display(&self) {
        let _ = self.sender.send(());
    }

    pub fn end(&self) {
        self.handle.abort();
    }
}
