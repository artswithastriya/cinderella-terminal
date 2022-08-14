use std::io::{stdout, Stdout};

use tui::{backend::CrosstermBackend, Terminal};

use crate::interface::Backend;

pub struct Crossterm;

impl Backend for Crossterm {
    fn events(tick_rate: std::time::Duration) -> std::sync::mpsc::Receiver<crate::events::Event> {
        todo!()
    }

    type TerminalArguments = CrosstermBackend<Stdout>;

    fn init_terminal() -> std::io::Result<Terminal<Self::TerminalArguments>> {
        let backend = CrosstermBackend::new(stdout());

        Ok(Terminal::new(backend)?)
    }
}
