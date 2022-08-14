#[cfg(feature = "crossterm")]
pub mod crossterm;
#[cfg(feature = "termion")]
pub mod termion;

pub mod auto {
    cfg_if::cfg_if! {
        if #[cfg(all(unix, feature = "termion"))] {
            pub use super::termion::*;
            pub use super::termion::Termion as Auto;
        } else if #[cfg(any(feature = "crossterm", windows))] {
            pub use super::crossterm::*;
            pub use super::crossterm::Crossterm as Auto;
        }
    }
}
