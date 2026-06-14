pub mod app;
pub mod client;
pub mod config;
pub mod event;
pub mod tui;
pub mod ui;

use std::time::Duration;

use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use futures::StreamExt;
use tokio::sync::mpsc::UnboundedSender;

use crate::config::Config;
use crate::event::AppEvent;
use crate::tui::Tui;

pub async fn run(_config: Config) -> Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<AppEvent>();

    let mut tui = Tui::new()?;

    spawn_event_task(tx.clone());
    spawn_tick_task(tx.clone());

    loop {
        tokio::select! {
            biased;
            Some(event) = rx.recv() => {
                match event {
                    AppEvent::Tick => {
                        tui.terminal.draw(|frame| {
                            ui::render(frame);
                        })?;
                    }
                    AppEvent::Key(key) => {
                        if key.code == KeyCode::Char('q') {
                            break;
                        }
                    }
                    AppEvent::Resize(_w, _h) => {
                        // resize handling (optional for now)
                    }
                    _ => {}
                }
            }
            _ = tokio::signal::ctrl_c() => {
                break;
            }
        }
    }

    tui.leave()?;
    Ok(())
}

fn spawn_event_task(tx: UnboundedSender<AppEvent>) {
    tokio::spawn(async move {
        let mut reader = crossterm::event::EventStream::new();
        loop {
            match reader.next().await {
                Some(Ok(Event::Key(key))) => {
                    let _ = tx.send(AppEvent::Key(key));
                }
                Some(Ok(Event::Resize(w, h))) => {
                    let _ = tx.send(AppEvent::Resize(w, h));
                }
                Some(Ok(_)) => {}
                Some(Err(_)) => break,
                None => break,
            }
        }
    });
}

fn spawn_tick_task(tx: UnboundedSender<AppEvent>) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(33));
        loop {
            interval.tick().await;
            if tx.send(AppEvent::Tick).is_err() {
                break;
            }
        }
    });
}
