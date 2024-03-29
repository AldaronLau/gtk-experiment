//!

// Asynchronous Client Thread
mod client;
// GTK Support
mod gtk;
// TODO: Add QT Support
// mod qt;

/// An action button.
///
/// Action buttons show up in the window titlebar.  At any given moment, the app
/// can have up to 5 action buttons, so they should be reserved for the most
/// common actions.
pub enum Action {
    /// Toggle open/closed vertical tab sidebar
    Sidebar,
    /// Create a new tab (file)
    New,
    /// Open tab from file
    Open,
    /// Synchronize file with server
    Sync,

    /// Go back to the first page
    First,
    /// Go back a page
    Backward,
    /// Go forward a page
    Forward,
    /// Go forward to the last page
    Last,
    /// Go to the home page
    Home,

    /// Reset the zoom level to 100%
    ZoomReset,
    /// Zoom out
    ZoomOut,
    /// Zoom in
    ZoomIn,
    /// Zoom to fit the window
    ZoomFit,

    /// Undo last action
    Undo,
    /// Redo last action
    Redo,

    /// Error Check, Spellcheck, Grammar Check, Counterpoint Check, etc.
    Check,
    /// Open Editor
    Edit,
    /// Select All
    Select,
    /// Copy
    Copy,
    /// Cut
    Cut,
    /// Paste
    Paste,

    /// Align Left
    AlignLeft,
    /// Align Center
    AlignCenter,
    /// Align Right
    AlignRight,
    /// Align Clamped (Justified)
    AlignClamp,

    /// Toggle Bold
    Bold,
    /// Toggle Italic
    Italic,
    /// Toggle Underlined
    Underline,
    /// Toggle Strikethrough
    Strikethrough,

    /// Outdent
    Outdent,
    /// Indent
    Indent,

    /// Flip object horizontally
    FlipH,
    /// Flip object vertically
    FlipV,
    /// Rotate object left
    RotateL,
    /// Rotate object right
    RotateR,

    /// Search/Find
    Search,

    /// Toggle Media Repeat
    Repeat,
    /// Shuffle Media
    Shuffle,
    /// Previous Media
    Previous,
    /// Rewind Media
    Rewind,
    /// Stop Media
    Stop,
    /// Pause Media
    Pause,
    /// Play Media
    Play,
    /// Record Media
    Record,
    /// Skip Through Media
    Skip,
    /// Next Media
    Next,
    /// Eject Media
    Eject,

    /// Insert/Add Image
    Image,
    /// Insert/Add Link
    Link,
    /// Insert/Add Object
    Object,
    /// Insert/Add Text
    Text,

    /// Edit Document Properties
    Page,

    /// Use a microphone
    Microphone,
    /// Use a webcam/camera
    Camera,
    /// Use a printer
    Printer,
    /// Use a scanner
    Scanner,
    /// Use a gamepad
    Gamepad,

    /// Jump to a pre-specified location
    Jump,
    /// Tag Version or Webpage (Bookmarks and File History)
    Tags,
    /// Add an item/page
    Add,
    /// Remove an item/page
    Remove,
    /// Share a document
    Share,
    /// View/Edit Account Details
    Account,
    /// View/Edit Settings
    Settings,
    /// View Help Page
    Help,
    /// Go into fullscreen
    Fullscreen,
    /// Leave fullscreen
    Restore,

    /// Look up something
    Look,
    /// Dropdown Menu
    Menu,
    /// Exit (like close, but doesn't close the program)
    Exit,
}

/// Messages received by the client.
#[derive(Debug)]
enum Event {
    /// Keyboard Input
    Key(u32),
    /// Text Input
    Text(char),
}

/// Message sent by the client.
enum Message {
    /// When the client requests the program to exit.
    Exit,
}

pub mod window {
    use pasts::prelude::*;

    pub enum Event {
        Test,
    }

    pub struct Window {
        sender: whisk::Channel<crate::Message>,
        recver: whisk::Channel<crate::Event>,
    }

    impl Notifier for Window {
        type Event = Event;

        fn poll_next(
            mut self: Pin<&mut Self>,
            exec: &mut Exec<'_>,
        ) -> Poll<Event> {
            if let Ready(event) = Pin::new(&mut self.recver).poll_next(exec) {
                // FIXME: Convert event, if sent to kbrd crate, call this
                // function recursively to get next event.
                let _ = event;
                Ready(Event::Test)
            } else {
                Pending
            }
        }
    }

    pub fn open<F: 'static>(user_thread: fn(Window) -> F)
    where
        F: Future<Output = ()>,
    {
        let recver = whisk::Channel::new();
        let sender = whisk::Channel::new();

        let tk_sender = recver.clone();
        let tk_recver = sender.clone();

        let window_sender = sender.clone();

        let mut window = Window { sender, recver };

        std::thread::spawn(move || {
            Executor::default().spawn(async move {
                user_thread(window);
                window_sender.send(crate::Message::Exit).await;
            });
        });

        crate::gtk::main(tk_sender, tk_recver);
    }
}
