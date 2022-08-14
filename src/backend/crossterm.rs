use async_std::task;
use futures::{select, FutureExt};
use futures_lite::stream::StreamExt;
use futures_timer::Delay;
use std::{
    io::{stdout, Stdout},
    sync::mpsc,
    thread,
};
use tui::{backend::CrosstermBackend, Terminal};

use crate::{events::Event, interface::Backend};

pub struct Crossterm;

impl Backend for Crossterm {
    fn events(tick_rate: std::time::Duration) -> mpsc::Receiver<crate::events::Event> {
        let (tx, rx) = mpsc::channel();
        let tx2 = tx.clone();
        let mut stream = crossterm::event::EventStream::new();
        task::spawn(async move {
            loop {
                let mut event = stream.next().fuse();
                let mut delay = Delay::new(tick_rate).fuse();
                select! {
                    _ = delay => {   if let Err(err) = tx.send(Event::Tick) {
                    eprintln!("{}", err);
                    break;
                } },
                    maybe_event = event => {
                        match maybe_event {
                            Some(Ok(event)) => {
                                tx.send(event.into());
                            }
                            Some(Err(e)) => println!("Error: {:?}\r", e),
                            None => break,
                        }
                    }
                };
            }
        });

        rx
    }

    type TerminalArguments = CrosstermBackend<Stdout>;

    fn init_terminal() -> std::io::Result<Terminal<Self::TerminalArguments>> {
        let backend = CrosstermBackend::new(stdout());

        Ok(Terminal::new(backend)?)
    }
}

impl From<crossterm::event::Event> for Event {
    fn from(event: crossterm::event::Event) -> Self {
        match event {
            crossterm::event::Event::Key(key) => Self::Keyboard(match key {
                _ => todo!()
            }),
            crossterm::event::Event::Mouse(_) => todo!(),
            crossterm::event::Event::Resize(_, _) => todo!(),
        }
    }
}
