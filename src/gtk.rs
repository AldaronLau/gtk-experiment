use super::{Action};
use gtk4::prelude::*;
use gtk4::Label;
use gtk4::{
    Application, ApplicationWindow, Box, Button, EventControllerKey, GLArea,
    HeaderBar, IMContextSimple, InputPurpose, Orientation,
};
use whisk::Channel;

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

        use std::os::raw::c_void;
        #[allow(unused)]
        pub const GL_ELEMENT_ARRAY_BUFFER: u32 = 0x8893;
        #[allow(unused)]
        pub const GL_ARRAY_BUFFER: u32 = 0x8892;
        #[allow(unused)]
        pub const GL_DYNAMIC_DRAW: u32 = 0x88E8;
        #[allow(unused)]
        pub type GLuint = u32;
        #[allow(unused)]
        pub type GLint = i32;
        #[allow(unused)]
        pub type GLenum = u32;
        #[allow(unused)]
        pub type GLboolean = u8;
        #[allow(unused)]
        pub type GLsizei = i32;
        #[allow(unused)]
        pub type GLchar = i8;
        #[allow(unused)]
        pub type GLbitfield = u32;
        #[allow(unused)]
        pub type GLsizeiptr = isize;
        #[allow(unused)]
        pub type GLfloat = f32;
        #[allow(unused)]
        pub type GLubyte = u8;
        #[allow(unused)]
pub const GL_FLOAT: u32 = 0x1406;
#[allow(unused)]
pub const GL_TEXTURE_2D: u32 = 0x0DE1;
#[allow(unused)]
pub const GL_TEXTURE_MAG_FILTER: u32 = 0x2800;
#[allow(unused)]
pub const GL_TEXTURE_MIN_FILTER: u32 = 0x2801;
#[allow(unused)]
pub const GL_NEAREST: i32 = 0x2600;
#[allow(unused)]
pub const GL_LINEAR: i32 = 0x2601;
#[allow(unused)]
pub const GL_LINEAR_MIPMAP_LINEAR: i32 = 0x2703;
#[allow(unused)]
pub const GL_NEAREST_MIPMAP_NEAREST: i32 = 0x2700;
#[allow(unused)]
pub const GL_NEAREST_MIPMAP_LINEAR: i32 = 0x2702;
#[allow(unused)]
pub const GL_RGBA: u32 = 0x1908;
#[allow(unused)]
pub const GL_UNSIGNED_BYTE: u32 = 0x1401;

extern "system" {
    fn glBindTexture(a: GLenum, b: GLuint) -> ();
    fn glBindBuffer(a: GLenum, b: GLuint) -> ();
    fn glBufferData(a: GLenum, b: GLsizeiptr, c: *const c_void, d: GLenum) -> ();
    fn glVertexAttribPointer(a: GLuint, b: GLint, c: GLenum, d: GLboolean, e: GLsizei, f: *const c_void) -> ();

    fn glDrawArrays(a: GLenum, b: GLint, c: GLsizei) -> ();
    // fn glViewport(a: GLint, b: GLint, c: GLsizei, d: GLsizei) -> ();
}

pub(super) fn main(send: Channel<crate::Event>, recv: Channel<crate::Message>) {
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

        let tabs = Button::from_icon_name(gtk_icon(&Action::Sidebar));
        header.pack_start(&tabs);

        let sidebar = Button::from_icon_name(gtk_icon(&Action::Backward));
        sidebar.connect_clicked(|_| {
            println!("back!");
        });
        header.pack_start(&sidebar);

        let sidebar = Button::from_icon_name(gtk_icon(&Action::Forward));
        sidebar.connect_clicked(|_| {
            println!("forward!");
        });
        header.pack_start(&sidebar);

        let new = Button::from_icon_name(gtk_icon(&Action::New));
        header.pack_start(&new);

        let search = Button::from_icon_name(gtk_icon(&Action::Search));
        let header2 = header.clone();
        let search_buffer = gtk4::EntryBuffer::new(Some("https://url.url"));
        let search_bar = gtk4::Entry::builder()
            .buffer(&search_buffer)
            .hexpand(true)
            .build();
        header.pack_start(&search);

        let menu = Button::from_icon_name(gtk_icon(&Action::Menu));
        let tags = Button::from_icon_name(gtk_icon(&Action::Tags));
        let sidebar = Button::from_icon_name(gtk_icon(&Action::Sync));
        let find = Button::from_icon_name(gtk_icon(&Action::Look));

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
            header2.set_title_widget(Option::<&gtk4::Widget>::None);
            header2.set_title_widget(Some(&search_bar2));
            // search_bar.set_focus(true);

            header2.set_title_widget(Option::<&gtk4::Widget>::None);
            header2.set_title_widget(Some(&title_widget2));
        });

        let ec_focus = gtk4::EventControllerFocus::new();
        let window2 = window.clone();
        let header2 = header.clone();
        let title_widget2 = title_widget.clone();
        ec_focus.connect_leave(move |f| {
            header2.set_title_widget(Option::<&gtk4::Widget>::None);
            header2.set_title_widget(Some(&title_widget2));
        });
        search_bar.add_controller(&ec_focus);

        header.set_title_widget(Option::<&gtk4::Widget>::None);
        header.set_title_widget(Some(&title_widget));
        header.set_decoration_layout(Some("menu:close"));

        window.set_titlebar(Some(&header));

        let column = Box::builder().orientation(Orientation::Vertical).build();

        let header_fullscreen_internal =
            HeaderBar::builder().show_title_buttons(false).build();
        let header_fullscreen = gtk4::Revealer::builder()
            .child(&header_fullscreen_internal)
            .reveal_child(false)
            .transition_type(gtk4::RevealerTransitionType::SlideDown)
            .build();
        let restore = Button::from_icon_name(gtk_icon(&Action::Restore));
        header_fullscreen_internal.pack_end(&restore);
        let menu = Button::from_icon_name(gtk_icon(&Action::Sidebar));
        header_fullscreen_internal.pack_end(&menu);

        let canvas = GLArea::builder()
            .auto_render(false)
            .has_depth_buffer(false)
            .hexpand(true)
            .use_es(true)
            .vexpand(true)
            .build();

        struct State {
            program: GLuint,
            position: GLint,
            texpos: GLint,
            position_buffer: GLuint,
            texpos_buffer: GLuint,
            texture: GLuint,
        }

        let state: *mut State = std::boxed::Box::leak(std::boxed::Box::new(State {
            program: 0,
            position: -1,
            texpos: -1,
            position_buffer: 0,
            texpos_buffer: 0,
            texture: 0,
        }));

        canvas.connect_realize(move |canvas| {
            let state = unsafe { &mut *state };

            canvas.make_current();

            extern "system" {

                fn glCreateShader(a: GLenum) -> GLuint;
                fn glShaderSource(a: GLuint, b: GLsizei, c: *const *const GLchar, d: *const GLint) -> ();
                fn glCompileShader(a: GLuint) -> ();
                fn glCreateProgram() -> GLuint;
                fn glAttachShader(a: GLuint, b: GLuint) -> ();
                fn glLinkProgram(a: GLuint) -> ();
                fn glDetachShader(a: GLuint, b: GLuint) -> ();
                fn glUseProgram(a: GLuint) -> ();
                fn glGetAttribLocation(a: GLuint, b: *const GLchar) -> GLint;
                fn glEnableVertexAttribArray(a: GLuint) -> ();
                
                fn glGenTextures(a: GLsizei, b: *mut GLuint) -> ();
                fn glTexParameteri(a: GLenum, b: GLenum, c: GLint) -> ();
                fn glTexImage2D(
                    a: GLenum,
                    b: GLint,
                    c: GLint,
                    d: GLsizei,
                    e: GLsizei,
                    f: GLint,
                    g: GLenum,
                    h: GLenum,
                    i: *const c_void,
                ) -> ();
                fn glGenBuffers(a: GLsizei, b: *mut GLuint) -> ();
            }
            // Compile shader program for rendering to canvas
            (state.program, state.position, state.texpos) = unsafe {
                const SHADER_TEX_VERT: &'static [u8] = b"\
                    #version 100\n\
                    precision mediump float;\n\
                    attribute vec2 position;\n\
                    attribute vec2 texpos;\n\
                    varying vec2 texcoord;\n\
                    void main() {\n\
                        gl_Position = vec4(position.xy, 0.0, 1.0);\n\
                        texcoord = texpos;\n\
                    }";
                const SHADER_TEX_FRAG: &'static [u8] = b"\
                    #version 100\n\
                    precision mediump float;\n\
                    uniform sampler2D texture;\n\
                    varying vec2 texcoord;\n\
                    void main() {\n\
                        gl_FragColor = texture2D(texture, texcoord);\n\
                    }";
                // Compile Vertex Shaders
                let v_shader = glCreateShader(0x8B31 /*vertex*/);
                glShaderSource(
                    v_shader,
                    1,
                    [SHADER_TEX_VERT.as_ptr() as *const _].as_ptr(),
                    [SHADER_TEX_VERT.len() as i32].as_ptr(),
                );
                glCompileShader(v_shader);
                // Compile Fragment Shader
                let f_shader = glCreateShader(0x8B30 /*fragment*/);
                glShaderSource(
                    f_shader,
                    1,
                    [SHADER_TEX_FRAG.as_ptr() as *const _].as_ptr(),
                    [SHADER_TEX_FRAG.len() as i32].as_ptr(),
                );
                glCompileShader(f_shader);
                // Link shaders together.
                let program = glCreateProgram();
                glAttachShader(program, v_shader);
                glAttachShader(program, f_shader);
                glLinkProgram(program);
                glDetachShader(program, v_shader);
                glDetachShader(program, f_shader);
                glUseProgram(program);
                // Get Vertex Attributes
                let position = glGetAttribLocation(program, b"position\0".as_ptr().cast());
                let texpos = glGetAttribLocation(program, b"texpos\0".as_ptr().cast());
                // Enable Vertex Attributes
                glEnableVertexAttribArray(position as u32);
                glEnableVertexAttribArray(texpos as u32);
                // 
                (program, position, texpos)
            };
            // Create canvas texture raster
            state.texture = unsafe {
                let mut texture = std::mem::MaybeUninit::uninit();
                glGenTextures(1, texture.as_mut_ptr());
                let texture = texture.assume_init();
                glBindTexture(GL_TEXTURE_2D, texture);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
                let pixels = vec![255u8; 640 * 360 * 4];
                glTexImage2D(
                    GL_TEXTURE_2D,
                    0,
                    GL_RGBA as i32,
                    640 as i32, // w
                    360 as i32, // h
                    0,
                    GL_RGBA,
                    GL_UNSIGNED_BYTE,
                    pixels.as_ptr().cast(),
                );
                texture
            };
            // Create Position Buffer
            type VertexList = [[f32; 2]; 6];
            state.position_buffer = unsafe {
                let position_c: VertexList = [
                    [-1.0, 1.0],
                    [-1.0, -1.0],
                    [1.0, -1.0],
                    [1.0, -1.0],
                    [1.0, 1.0],
                    [-1.0, 1.0],
                ];
                let mut buffer = std::mem::MaybeUninit::uninit();
                glGenBuffers(1, buffer.as_mut_ptr());
                let buffer = buffer.assume_init();
                glBindBuffer(GL_ARRAY_BUFFER, buffer);
                glBufferData(
                    GL_ARRAY_BUFFER,
                    std::mem::size_of::<VertexList>() as isize,
                    position_c.as_ptr().cast(),
                    GL_DYNAMIC_DRAW,
                );
                glVertexAttribPointer(state.position as GLuint, 2, GL_FLOAT, 0, 0, std::ptr::null());
                buffer
            };
            // Create Texture Coordinates Buffer
            state.texpos_buffer = unsafe {
                let texpos_c: VertexList = [
                    [0.0, 1.0],
                    [0.0, 0.0],
                    [1.0, 0.0],
                    [1.0, 0.0],
                    [1.0, 1.0],
                    [0.0, 1.0],
                ];
                let mut buffer = std::mem::MaybeUninit::uninit();
                glGenBuffers(1, buffer.as_mut_ptr());
                let buffer = buffer.assume_init();
                glBindBuffer(GL_ARRAY_BUFFER, buffer);
                glBufferData(
                    GL_ARRAY_BUFFER,
                    std::mem::size_of::<VertexList>() as isize,
                    texpos_c.as_ptr().cast(),
                    GL_DYNAMIC_DRAW,
                );
                glVertexAttribPointer(state.texpos as GLuint, 2, GL_FLOAT, 0, 0, std::ptr::null());
                buffer
            };
        });

        canvas.connect_unrealize(move |canvas| {
            let state = unsafe { &mut *state };
            
            extern "system" {
                fn glDeleteProgram(a: GLuint) -> ();
                fn glDeleteBuffers(a: GLsizei, b: *const GLuint) -> ();
                fn glDeleteTextures(a: GLsizei, b: *const GLuint) -> ();
            }
            unsafe {
                glDeleteProgram(state.program);
                glDeleteTextures(1, [state.texture].as_ptr());
                glDeleteBuffers(2, [state.position_buffer, state.texpos_buffer].as_ptr());
            }
        });

        let win = window.clone();
        canvas.connect_render(move |canvas, context| {
            let state = unsafe { &mut *state };

            // println!("render");
            // dbg!(canvas.width(), canvas.height());

            #[link(name = "GLESv2")]
            extern "C" {
                fn glClearColor(r: f32, g: f32, b: f32, a: f32);
                fn glClear(field: std::os::raw::c_uint);
                fn glFlush();
            }
            unsafe {
                glClearColor(0.0, 0.0, 0.0, 1.0);
                glClear(0x00004000);
                // Bind position_buffer to position
                glBindBuffer(GL_ARRAY_BUFFER, state.position_buffer);
                glVertexAttribPointer(state.position as GLuint, 2, GL_FLOAT, 0, 0, std::ptr::null());
                // Bind texpos_buffer to texpos
                glBindBuffer(GL_ARRAY_BUFFER, state.texpos_buffer);
                glVertexAttribPointer(state.texpos as GLuint, 2, GL_FLOAT, 0, 0, std::ptr::null());
                glBindTexture(GL_TEXTURE_2D, state.texture);
                glDrawArrays(
                    /* Triangles */ 0x0004 as GLuint,
                    /* Start */ 0 as GLint,
                    /* End */ 6 as GLsizei,
                );
                glFlush();
            }

            gtk4::Inhibit(false)
        });

        canvas.add_tick_callback(|canvas, frame_clock| {
            // let micros = std::time::Duration::from_micros(frame_clock.frame_time());
            // dbg!(micros);
            canvas.queue_render();
            glib::source::Continue(true) // Keep running
        });

        let keyboard = EventControllerKey::new();
        keyboard.connect_modifiers(|controller, modtype| {
            dbg!(modtype);
            gtk4::Inhibit(false)
        });
        keyboard.connect_key_pressed(|controller, keyval, keycode, state| {
            dbg!(keyval.to_unicode());

            gtk4::Inhibit(false)
        });
        keyboard.connect_key_released(|controller, keyval, keycode, state| {
            dbg!(keyval, keycode, state);
        });
        window.add_controller(&keyboard);

        column.prepend(&canvas);
        column.prepend(&header_fullscreen);
        header_fullscreen.hide();

        let win = window.clone();
        let h_f = header_fullscreen.clone();
        restore.connect_clicked(move |f| {
            h_f.hide();
            win.unfullscreen();
        });

        let win = window.clone();
        let h_f = header_fullscreen.clone();
        // FIXME: Enter fullscreen.
        /*fullscreen.connect_clicked(move |f| {
            h_f.show();
            win.fullscreen();
        });*/

        window.set_child(Some(&column));
        window.show();
    });

    std::process::exit(application.run());
}
