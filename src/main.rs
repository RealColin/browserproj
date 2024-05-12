use glow::*;

mod shader;

type Color = (u8, u8, u8);

struct State {
    tabs: Vec<Tab>,
    current_tab: usize,
    win_width: f32,
    win_height: f32,
}

impl State {
    fn new() -> Self {
        let mut tabs = Vec::new();
        let default = Tab::new();
        tabs.push(default);

        State {
            tabs,
            current_tab: 0,
            win_width: 800.0,
            win_height: 600.0,
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
    let mut state = State::new();

    let context = create_sdl2_context();

    let gl = context.0;
    let window = context.1;
    let mut events_loop = context.2;

    let shader = shader::Shader::build(&gl, "res/shaders/basic.vert", "res/shaders/basic.frag");
    shader.activate(&gl);

    let _interval = window.subsystem().gl_set_swap_interval(1);

    let vertices: [f32; 6] = [-0.5, -0.5, 0.5, -0.5, 0.0, 0.5];

    let vao = unsafe {
        let vert_ptr: &[u8] = core::slice::from_raw_parts(
            vertices.as_ptr() as *const u8,
            vertices.len() * core::mem::size_of::<f32>(),
        );

        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vert_ptr, glow::STATIC_DRAW);

        let vao = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(vao));
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 2 * 4, 0);

        vao
    };

    'render: loop {
        let should_close = handle_events(&gl, &mut events_loop, &mut state);

        if should_close {
            break 'render;
        }

        unsafe {
            gl.clear_color(1.0, 1.0, 1.0, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            shader.activate(&gl);
            gl.bind_vertex_array(Some(vao));
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
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

fn handle_events(gl: &glow::Context, events: &mut sdl2::EventPump, state: &mut State) -> bool {
    for event in events.poll_iter() {
        if let sdl2::event::Event::Quit { .. } = event {
            return true;
        }

        if let sdl2::event::Event::Window { win_event, .. } = event {
            match win_event {
                sdl2::event::WindowEvent::Resized(x, y) => unsafe {
                    state.win_width = x as f32;
                    state.win_height = y as f32;
                    gl.viewport(0, 0, x, y);
                },

                _ => {}
            }
        }
    }

    false
}
