use futures_util::stream::{StreamExt, StreamFuture};
use gtk4::prelude::*;
use gtk4::Label;
use gtk4::{
    Application, ApplicationWindow, Box, Button, EventControllerKey, GLArea,
    HeaderBar, IMContextSimple, InputPurpose, Orientation,
};

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

fn gtk_icon(action: &Action) -> &str {
    match action {
        Action::Sidebar => "open-menu-symbolic",
        Action::New => "tab-new-symbolic",
        Action::Open => "document-open-symbolic",
        Action::Sync => "emblem-synchronizing-symbolic",

        Action::First => "go-first-symbolic",
        Action::Backward => "go-previous-symbolic",
        Action::Forward => "go-next-symbolic",
        Action::Last => "go-last-symbolic",
        Action::Home => "go-home-symbolic",

        Action::ZoomReset => "zoom-original-symbolic",
        Action::ZoomOut => "zoom-out-symbolic",
        Action::ZoomIn => "zoom-in-symbolic",
        Action::ZoomFit => "zoom-fit-best-symbolic",

        Action::Undo => "edit-undo-symbolic",
        Action::Redo => "edit-redo-symbolic",

        Action::Check => "error-correct-symbolic",
        Action::Edit => "document-edit-symbolic",
        Action::Select => "edit-select-all-symbolic",
        Action::Copy => "edit-copy-symbolic",
        Action::Cut => "edit-cut-symbolic",
        Action::Paste => "edit-paste-symbolic",

        Action::AlignLeft => "format-justify-left-symbolic",
        Action::AlignCenter => "format-justify-center-symbolic",
        Action::AlignRight => "format-justify-right-symbolic",
        Action::AlignClamp => "format-justify-fill-symbolic",

        Action::Bold => "format-text-bold-symbolic",
        Action::Italic => "format-text-italic-symbolic",
        Action::Underline => "format-text-underline-symbolic",
        Action::Strikethrough => "format-text-strikethrough-symbolic",

        Action::Outdent => "format-indent-less-symbolic",
        Action::Indent => "format-indent-more-symbolic",

        Action::FlipH => "object-flip-horizontal",
        Action::FlipV => "object-flip-vertical",
        Action::RotateL => "object-rotate-left-symbolic",
        Action::RotateR => "object-rotate-right-symbolic",

        Action::Search => "system-search-symbolic",

        Action::Repeat => "media-playlist-repeat-symbolic",
        Action::Shuffle => "media-playlist-shuffle-symbolic",
        Action::Previous => "media-skip-backward-symbolic",
        Action::Rewind => "media-seek-backward-symbolic",
        Action::Stop => "media-playback-stop-symbolic",
        Action::Pause => "media-playback-pause-symbolic",
        Action::Play => "media-playback-start-symbolic",
        Action::Record => "media-record-symbolic",
        Action::Skip => "media-seek-forward-symbolic",
        Action::Next => "media-skip-forward-symbolic",
        Action::Eject => "media-eject-symbolic",

        Action::Image => "insert-image-symbolic",
        Action::Link => "insert-link-symbolic",
        Action::Object => "insert-object-symbolic",
        Action::Text => "insert-text-symbolic",

        Action::Page => "document-properties-symbolic",

        Action::Microphone => "audio-input-microphone-symbolic",
        Action::Camera => "camera-web-symbolic",
        Action::Printer => "printer-symbolic",
        Action::Scanner => "scanner-symbolic",
        Action::Gamepad => "input-gaming-symbolic",

        Action::Jump => "go-jump-symbolic",
        Action::Tags => "document-save-as-symbolic",
        Action::Add => "list-add-symbolic",
        Action::Remove => "list-remove-symbolic",
        Action::Share => "send-to-symbolic",
        Action::Account => "avatar-default-symbolic",
        Action::Settings => "preferences-other-symbolic",
        Action::Help => "help-browser-symbolic",
        Action::Fullscreen => "view-fullscreen-symbolic",
        Action::Restore => "view-restore-symbolic",

        Action::Look => "go-jump-symbolic",
        Action::Menu => "view-more-symbolic",
        Action::Exit => "application-exit-symbolic",
    }
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

/// Client State
struct State<'a> {
    stream: StreamFuture<flume::r#async::RecvStream<'a, Event>>,
}

// Exit type for State.
type Exit = ();

impl<'a> State<'a> {
    fn event(
        &mut self,
        (event, stream): (Option<Event>, flume::r#async::RecvStream<'a, Event>),
    ) -> std::task::Poll<Exit> {
        dbg!(event);
        self.stream = stream.into_future();
        std::task::Poll::Pending
    }
}

async fn start(sender: flume::Sender<Message>, recver: flume::Receiver<Event>) {
    let mut state = State {
        stream: recver.stream().into_future(),
    };

    pasts::Loop::new(&mut state)
        .when(|s| &mut s.stream, State::event)
        .await;

    sender.send(Message::Exit).unwrap();
}

fn main() {
    let (send, client_recv) = flume::bounded(1);
    let (client_send, recv) = flume::bounded(1);

    std::thread::spawn(move || {
        pasts::block_on(start(client_send, client_recv));
    });

    let application = Application::builder()
        .application_id("com.aldaronlau.gtk-test")
        .build();

    application.connect_activate(|app| {
        // We create the main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(640)
            .default_height(360)
            .title("Hello, World")
            .build();
        window.maximize();

        // Specialized keyboard types
        let input = IMContextSimple::new();
        input.set_input_purpose(InputPurpose::FreeForm); // text

        // input.set_input_purpose(InputPurpose::Email).build(); // email
        // input.set_input_purpose(InputPurpose::Url).build(); // url
        // input.set_input_purpose(InputPurpose::Phone).build(); // phone
        // input.set_input_purpose(InputPurpose::Digits).build(); // pin

        let header = HeaderBar::new();

        let tabs = Button::from_icon_name(Some(gtk_icon(&Action::Sidebar)));
        header.pack_start(&tabs);

        let sidebar = Button::from_icon_name(Some(gtk_icon(&Action::Backward)));
        sidebar.connect_clicked(|_| {
            println!("back!");
        });
        header.pack_start(&sidebar);

        let sidebar = Button::from_icon_name(Some(gtk_icon(&Action::Forward)));
        sidebar.connect_clicked(|_| {
            println!("forward!");
        });
        header.pack_start(&sidebar);

        let new = Button::from_icon_name(Some(gtk_icon(&Action::New)));
        header.pack_start(&new);

        let search = Button::from_icon_name(Some(gtk_icon(&Action::Search)));
        let header2 = header.clone();
        let search_buffer = gtk4::EntryBuffer::new(Some("https://url.url"));
        let search_bar = gtk4::Entry::builder()
            .buffer(&search_buffer)
            .hexpand(true)
            //.search_mode_enabled(true)
            //.show_close_button(true)
            .build();
        header.pack_start(&search);

        let menu = Button::from_icon_name(Some(gtk_icon(&Action::Menu)));
        let tags = Button::from_icon_name(Some(gtk_icon(&Action::Tags)));
        let sidebar = Button::from_icon_name(Some(gtk_icon(&Action::Sync)));
        let find = Button::from_icon_name(Some(gtk_icon(&Action::Look)));

        header.pack_end(&menu);
        header.pack_end(&sidebar);
        header.pack_end(&tags);
        header.pack_end(&find);

        let title_widget = {
            let title_widget = Box::builder()
                .orientation(Orientation::Vertical)
                .hexpand(true)
                .valign(gtk4::Align::Center)
                .build();

            let title = Label::new(Some("Page Title"));
            title.style_context().add_class("title");
            let subtitle = Label::new(Some("https://url.url"));
            subtitle.style_context().add_class("subtitle");

            title_widget.append(&title);
            title_widget.append(&subtitle);

            title_widget
        };

        let search_bar2 = search_bar.clone();
        let header2 = header.clone();
        let title_widget2 = title_widget.clone();
        search.connect_clicked(move |_| {
            header2.set_title_widget::<gtk4::Widget>(None);
            header2.set_title_widget(Some(&search_bar2));
            // search_bar.set_focus(true);

            std::thread::sleep(std::time::Duration::from_millis(500));

            header2.set_title_widget::<gtk4::Widget>(None);
            header2.set_title_widget(Some(&title_widget2));
        });

        let ec_focus = gtk4::EventControllerFocus::new();
        let window2 = window.clone();
        let header2 = header.clone();
        let title_widget2 = title_widget.clone();
        ec_focus.connect_leave(move |f| {
            header2.set_title_widget::<gtk4::Widget>(None);
            header2.set_title_widget(Some(&title_widget2));
        });
        search_bar.add_controller(&ec_focus);

        header.set_title_widget::<gtk4::Widget>(None);
        header.set_title_widget(Some(&title_widget));
        header.set_decoration_layout(Some("menu:close"));

        window.set_titlebar(Some(&header));

        let column = Box::builder().orientation(Orientation::Vertical).build();

        window.set_child(Some(&column));

        let header_fullscreen_internal =
            HeaderBar::builder().show_title_buttons(false).build();
        let header_fullscreen = gtk4::Revealer::builder()
            .child(&header_fullscreen_internal)
            .reveal_child(false)
            .transition_type(gtk4::RevealerTransitionType::SlideDown)
            .build();
        let restore = Button::from_icon_name(Some(gtk_icon(&Action::Restore)));
        header_fullscreen_internal.pack_end(&restore);
        let menu = Button::from_icon_name(Some(gtk_icon(&Action::Sidebar)));
        header_fullscreen_internal.pack_end(&menu);

        let canvas = GLArea::builder()
            .auto_render(true)
            .has_depth_buffer(false)
            .use_es(true)
            .hexpand(true)
            .vexpand(true)
            .build();

        canvas.connect_render(|canvas, context| {
            dbg!(canvas.width(), canvas.height());

            #[link(name = "GLESv2")]
            extern "C" {
                fn glClearColor(r: f32, g: f32, b: f32, a: f32);
                fn glClear(field: std::os::raw::c_uint);
            }
            unsafe {
                glClearColor(0.25, 0.25, 0.25, 1.0);
                glClear(0x00004000);
            }
            gtk4::Inhibit(true)
        });

        let keyboard = EventControllerKey::new();
        keyboard.connect_modifiers(|controller, modtype| {
            dbg!(modtype);
            gtk4::Inhibit(true)
        });
        keyboard.connect_key_pressed(|controller, keyval, keycode, state| {
            dbg!(keyval.to_unicode());

            gtk4::Inhibit(true)
        });
        keyboard.connect_key_released(|controller, keyval, keycode, state| {
            dbg!(keyval, keycode, state);
        });
        window.add_controller(&keyboard);

        column.prepend(&canvas);
        column.prepend(&header_fullscreen);
        header_fullscreen.hide();

        let win = window.clone();
        let col = column.clone();
        let h_f = header_fullscreen.clone();
        restore.connect_clicked(move |f| {
            h_f.hide();
            win.unfullscreen();
        });

        let win = window.clone();
        let col = column.clone();
        let h_f = header_fullscreen.clone();
        // FIXME: Enter fullscreen.
        /*fullscreen.connect_clicked(move |f| {
            h_f.show();
            win.fullscreen();
        });*/

        window.show();
    });

    std::process::exit(application.run());
}
