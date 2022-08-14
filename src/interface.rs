use std::time::Duration;

#[cfg(feature = "tui")]
use tui::Terminal;

use crate::events::Event;

pub trait Backend {
    fn events(tick_rate: Duration) -> std::sync::mpsc::Receiver<Event>;

    cfg_if::cfg_if! {
    if #[cfg(feature = "tui")]{
       type TerminalArguments: tui::backend::Backend;

       fn init_terminal() -> std::io::Result<Terminal<Self::TerminalArguments>>;
    }
    }
}
