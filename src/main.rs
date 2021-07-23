use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, HeaderBar, Button, IconSize};

/// An action button.
///
/// Action buttons show up in the window titlebar.  At any given moment, the app
/// can have up to 8 action buttons, so they should be reserved for the most
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

    /// Go back to the first page.
    First,
    /// Go back a page.
    Backward,
    /// Go forward a page.
    Forward,
    /// Go forward to the last page.
    Last,
    /// Go to the home page.
    Home,

    /// Reset the zoom level to 100%.
    ZoomReset,
    /// Zoom out
    ZoomOut,
    /// Zoom in
    ZoomIn,
    /// Zoom to fit the window.
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
    /// Pin File History (and restore)
    Pin,
    /// Add an item/page.
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
    /// Go into fullscreen.
    Fullscreen,
    /// Leave fullscreen.
    Restore,
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
        Action::Pin => "view-pin-symbolic",
        Action::Add => "list-add-symbolic",
        Action::Remove => "list-remove-symbolic",
        Action::Share => "send-to-symbolic",
        Action::Account => "avatar-default-symbolic",
        Action::Settings => "emblem-system-symbolic",
        Action::Help => "help-browser-symbolic",
        Action::Fullscreen => "view-fullscreen-symbolic",
        Action::Restore => "view-restore-symbolic",
    }
}

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(640)
            .default_height(360)
            .title("Hello, World!")
            .build();
        win.maximize();
        // win.fullscreen();
        win.set_titlebar(Some(
            &HeaderBar::builder()
                .title("Title")
                .subtitle("Subtitle")
                .show_close_button(true)
                .child(&Button::from_icon_name(Some("document-open"), IconSize::Button))
                .child(&Button::from_icon_name(Some("view-refresh"), IconSize::Button))
                .child(&Button::from_icon_name(Some("edit-undo"), IconSize::Button))
                .child(&Button::from_icon_name(Some("edit-redo-symbolic"), IconSize::Button))
                .build(),
        ));

        // Don't forget to make all widgets visible.
        win.show_all();
    });

    app.run();
}