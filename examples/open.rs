// FIXME Test
use gtk_test::window;

use pasts::{prelude::*, Join};
use window::Window;

struct App {
    window: Window,
}

impl App {
    fn event(&mut self, _event: window::Event) -> Poll<()> {
        Ready(())
    }

    async fn open(window: Window) {
        let mut app = Self { window };

        Join::new(&mut app).on(|s| &mut s.window, Self::event).await;
    }
}

fn main() {
    window::open(App::open);
}
