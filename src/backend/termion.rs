use std::io::Stdout;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use std::{io, sync::mpsc, thread};
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::Terminal;

use termion::input::TermRead;
pub struct Termion;

impl Backend for Termion {
    type TerminalArguments = TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>;

    fn init_terminal() -> Result<Terminal<Self::TerminalArguments>, std::io::Error> {
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(terminal)
    }

    fn events(tick_rate: Duration) -> Receiver<crate::events::Event> {
        let (tx, rx) = mpsc::channel();
        let keys_tx = tx.clone();
        thread::spawn(move || {
            let stdin = io::stdin();
            for key in stdin.keys().flatten() {
                if let Err(err) = keys_tx.send(Event::Input(key.into())) {
                    eprintln!("{}", err);
                    return;
                }
            }
        });
        thread::spawn(move || loop {
            if let Err(err) = tx.send(Event::Tick) {
                eprintln!("{}", err);
                break;
            }
            thread::sleep(tick_rate);
        });
        rx
    }
}

impl From<termion::event::Key> for Key {
    fn from(key: termion::event::Key) -> Self {
        use termion::event::Key::*;
        match key {
            Backspace => Self::Backspace,
            Left => Self::Left,
            Right => Self::Right,
            Up => Self::Up,
            Down => Self::Down,
            Home => Self::Home,
            End => Self::End,
            PageUp => Self::PageUp,
            PageDown => Self::PageDown,
            BackTab => Self::BackTab,
            Delete => Self::Delete,
            Insert => Self::Insert,
            F(key) => Self::F(key),
            Char(char) => Self::Char(char),
            Alt(alt) => Self::Alt(alt),
            Ctrl(char) => Self::Ctrl(char),
            Null => Self::Null,
            Esc => Self::Esc,
            key => panic!("{key:?} is not mappable to ctermderella's Key enum!!"),
        }
    }
}

use crate::events::{Event, Key};
use crate::interface::Backend;
