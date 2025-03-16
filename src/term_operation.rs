use crossterm::event::{Event as TermEvent, EventStream, KeyCode, KeyEvent, KeyEventKind};
use futures::{FutureExt, StreamExt};

use crate::event::{
    mino_operation::{Direction, MinoOperation},
    Event, EventSender,
};

pub async fn term_operation(event_sender: EventSender) {
    let mut term_event_reader = EventStream::new();

    loop {
        let term_event = term_event_reader.next().fuse().await;
        if let Some(Ok(term_event)) = term_event {
            match term_event {
                TermEvent::Key(key_event) => {
                    key_pressed(&event_sender, key_event).await;
                }
                TermEvent::Resize(_, _) => event_sender.send(Event::DisplayAll).await,
                _ => {}
            }
        }
    }
}

async fn key_pressed(event_sender: &EventSender, key_event: KeyEvent) {
    if key_event.kind == KeyEventKind::Press {
        match key_event.code {
            KeyCode::Esc => event_sender.send(Event::End).await,
            KeyCode::Right => {
                event_sender
                    .send(Event::MinoOperation(MinoOperation::Move(Direction::Right)))
                    .await
            }
            KeyCode::Left => {
                event_sender
                    .send(Event::MinoOperation(MinoOperation::Move(Direction::Left)))
                    .await
            }
            KeyCode::Down => {
                event_sender
                    .send(Event::MinoOperation(MinoOperation::SoftDrop))
                    .await
            }
            KeyCode::Char('z') => {
                event_sender
                    .send(Event::MinoOperation(MinoOperation::RotateLeft))
                    .await
            }
            KeyCode::Char('x') => {
                event_sender
                    .send(Event::MinoOperation(MinoOperation::RotateRight))
                    .await
            }
            KeyCode::Char('c') => {
                event_sender
                    .send(Event::MinoOperation(MinoOperation::Hold))
                    .await
            }
            KeyCode::Char(' ') => {
                event_sender
                    .send(Event::MinoOperation(MinoOperation::HardDrop))
                    .await
            }
            _ => {}
        }
    }
}
