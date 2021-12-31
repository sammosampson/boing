mod loops;
mod channel;
mod producer; 

pub use shrev::{
    EventChannel, ReaderId, EventIterator
};
pub use loops::*;
pub use channel::*;
pub use producer::*;

use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum SystemEvent {
    CloseRequested,
    KeyboardAction { state: InputState, button: KeyboardButton }
}