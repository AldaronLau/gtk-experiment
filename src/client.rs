//! Asynchronous Client Thread
//!
//! Sends messages to the main thread for graphics and headerbar support, and
//! receives user input events.

use super::{Event, Message};
use flume::r#async::RecvStream;
use flume::{Receiver, Sender};
use futures_util::stream::{StreamExt, StreamFuture};
use pasts::Loop;
use std::task::Poll::{self, Pending};

/// Client State
struct State<'a> {
    stream: StreamFuture<flume::r#async::RecvStream<'a, Event>>,
}

// Exit type for State.
type Exit = ();

impl<'a> State<'a> {
    fn event(
        &mut self,
        (event, stream): (Option<Event>, RecvStream<'a, Event>),
    ) -> Poll<Exit> {
        self.stream = stream.into_future();
        Pending
    }
}

pub(super) async fn start(sender: Sender<Message>, recver: Receiver<Event>) {
    let mut state = State {
        stream: recver.stream().into_future(),
    };

    Loop::new(&mut state)
        .when(|s| &mut s.stream, State::event)
        .await;

    sender.send(Message::Exit).unwrap();
}
