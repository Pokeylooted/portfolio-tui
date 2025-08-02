//! Event handling module for future use
//! 
//! This module provides event handling capabilities for the application.
//! It is currently not used but is kept for future enhancements.
//! The warnings are suppressed with the #[allow(dead_code)] attribute.

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

/// Terminal events
#[allow(dead_code)]
pub enum Event {
    /// Key press
    Key(KeyEvent),
    /// Tick
    Tick,
}

/// Event handler
#[allow(dead_code)]
pub struct EventHandler {
    /// Event sender
    sender: mpsc::Sender<Event>,
    /// Event receiver
    receiver: mpsc::Receiver<Event>,
    /// Event handler thread
    handler: thread::JoinHandle<()>,
}

#[allow(dead_code)]
impl EventHandler {
    /// Create a new event handler with the given tick rate
    pub fn new(tick_rate: Duration) -> Self {
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or_else(|| Duration::from_secs(0));

                    if event::poll(timeout).unwrap() {
                        if let CrosstermEvent::Key(key) = event::read().unwrap() {
                            if sender.send(Event::Key(key)).is_err() {
                                break;
                            }
                        }
                    }

                    if last_tick.elapsed() >= tick_rate {
                        if sender.send(Event::Tick).is_err() {
                            break;
                        }
                        last_tick = Instant::now();
                    }
                }
            })
        };

        Self {
            sender,
            receiver,
            handler,
        }
    }

    /// Receive the next event
    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.receiver.recv()
    }
}