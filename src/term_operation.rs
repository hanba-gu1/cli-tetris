use std::{collections::HashMap, time::Duration};

use crossterm::event::{Event as TermEvent, EventStream, KeyCode, KeyEvent, KeyEventKind};
use futures::{FutureExt, StreamExt};

use crate::event::{self, mino_operation::Direction, Event, EventSender};

pub(super) async fn term_operation(event_sender: EventSender) {
    let mut key_event_handles = HashMap::new();
    let mut term_event_reader = EventStream::new();

    loop {
        let term_event = term_event_reader.next().fuse().await;
        if let Some(Ok(term_event)) = term_event {
            match term_event {
                TermEvent::Key(key_event) => {
                    key_pressed(&event_sender, &mut key_event_handles, key_event).await;
                }
                TermEvent::Resize(_, _) => event_sender.send(Event::DisplayAll).await,
                _ => {}
            }
        }
    }
}

async fn key_pressed(
    event_sender: &EventSender,
    key_event_handles: &mut HashMap<KeyCode, Option<tokio::task::JoinHandle<()>>>,
    key_event: KeyEvent,
) {
    use event::mino_operation::MinoOperation::*;

    let move_repeat_duration = (Duration::from_millis(300), Duration::from_millis(50));

    match key_event.kind {
        KeyEventKind::Press => {
            let event = match key_event.code {
                KeyCode::Esc => Some((Event::End, None)),
                KeyCode::Left => Some((
                    Event::MinoOperation(Move(Direction::Left)),
                    Some(move_repeat_duration),
                )),
                KeyCode::Right => Some((
                    Event::MinoOperation(Move(Direction::Right)),
                    Some(move_repeat_duration),
                )),
                KeyCode::Down => Some((Event::MinoOperation(StartSoftDrop), None)),
                KeyCode::Char('z') => Some((Event::MinoOperation(RotateLeft), None)),
                KeyCode::Char('x') => Some((Event::MinoOperation(RotateRight), None)),
                KeyCode::Char('c') => Some((Event::MinoOperation(Hold), None)),
                KeyCode::Char(' ') => Some((Event::MinoOperation(HardDrop), None)),
                _ => None,
            };

            if let Some((event, repeat_duration)) = event {
                if !key_event_handles.contains_key(&key_event.code) {
                    event_sender.send(event).await;
                    if let Some((first_duration, repeat_duration)) = repeat_duration {
                        let event_sender = event_sender.clone();
                        let key_event_handle = tokio::spawn(async move {
                            tokio::time::sleep(first_duration).await;
                            loop {
                                event_sender.send(event).await;
                                tokio::time::sleep(repeat_duration).await;
                            }
                        });
                        key_event_handles.insert(key_event.code, Some(key_event_handle));
                    } else {
                        key_event_handles.insert(key_event.code, None);
                    }
                }
            }
        }
        KeyEventKind::Release => {
            match key_event.code {
                KeyCode::Down => event_sender.send(Event::MinoOperation(EndSoftDrop)).await,
                _ => {}
            }
            if let Some(Some(key_event_handle)) = key_event_handles.remove(&key_event.code) {
                key_event_handle.abort();
            }
        }
        _ => {}
    }
}
