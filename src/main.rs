use glow::*;

type Color = (u8, u8, u8);

struct State {
    tabs: Vec<Tab>,
    current_tab: usize,
}

impl State {
    fn new() -> Self {
        let mut tabs = Vec::new();
        let default = Tab::new();
        tabs.push(default);

        State {
            tabs,
            current_tab: 0,
        }
    }
}

struct Tab {
    pages: Vec<Page>,
    current_page: usize,
}

impl Tab {
    fn new() -> Self {
        let mut pages = Vec::new();
        let default = Page::new(String::from("New tab"));
        pages.push(default);

        Tab {
            pages,
            current_page: 0,
        }
    }
}

struct Page {
    elements: Vec<Element>,
    background_color: Color,
    name: String,
}

impl Page {
    fn new(name: String) -> Self {
        let elements = Vec::new();

        Page {
            elements,
            background_color: (100, 100, 100),
            name,
        }
    }
}

struct Element {}

fn main() {
    let state = State::new();
    let context = create_sdl2_context();

    let gl = context.0;
    let window = context.1;
    let mut events_loop = context.2;

    let _interval = window.subsystem().gl_set_swap_interval(1);

    'render: loop {
        let should_close = handle_events(&mut events_loop);

        if should_close {
            break 'render;
        }

        unsafe {
            gl.clear_color(1.0, 1.0, 1.0, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }

        window.gl_swap_window();
    }
}

fn create_sdl2_context() -> (
    glow::Context,
    sdl2::video::Window,
    sdl2::EventPump,
    sdl2::video::GLContext,
    sdl2::Sdl,
) {
    unsafe {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 6);
        gl_attr.set_context_flags().forward_compatible().set();

        let window = video
            .window("Browser", 800, 600)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let gl_context = window.gl_create_context().unwrap();
        let gl = glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);
        let event_loop = sdl.event_pump().unwrap();

        (gl, window, event_loop, gl_context, sdl)
    }
}

fn handle_events(events: &mut sdl2::EventPump) -> bool {
    for event in events.poll_iter() {
        if let sdl2::event::Event::Quit { .. } = event {
            return true;
        }
    }

    false
}
