//! Asynchronous Client Thread
//!
//! Sends messages to the main thread for graphics and headerbar support, and
//! receives user input events.

use super::{Event, Message};
use pasts::Join;
use std::task::Poll::{self, Pending};
use whisk::Channel;

/// Client State
struct State {
    recver: Channel<Event>,
}

// Exit type for State.
type Exit = ();

impl State {
    fn event(
        &mut self,
        event: Event,
    ) -> Poll<Exit> {
        Pending
    }
}

pub(super) async fn start(sender: Channel<Message>, recver: Channel<Event>) {
    let mut state = State { recver };

    Join::new(&mut state)
        .on(|s| &mut s.recver, State::event)
        .await;

    sender.send(Message::Exit).await;
}
