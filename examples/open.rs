// FIXME Test
use gtk_test::window;

use window::{Window, Loop, Poll, Pending, Ready};

struct App {
    window: Window,
}

impl App {
    fn event(&mut self, _event: window::Event) -> Poll<()> {
        Ready(())
    }

    async fn open(window: Window) {
        let mut app = Self {
            window,
        };

        Loop::new(&mut app).when(|s| &mut s.window, Self::event).await;
    }
}

fn main() {
    window::open(App::open);
}
