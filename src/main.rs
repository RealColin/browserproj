use glow::*;

mod shader;

type Color = (u8, u8, u8);

struct State {
    win_width: f32,
    win_height: f32,
    vao: NativeVertexArray,
}

fn gen_vao(win_width: f32, win_height: f32, gl: &glow::Context) -> NativeVertexArray {
    let top_left = pixel_to_ndc(win_width, win_height, 10.0, 5.0);
    let top_right = pixel_to_ndc(win_width, win_height, 235.0, 5.0);
    let bottom_left = pixel_to_ndc(win_width, win_height, 10.0, 40.0);
    let bottom_right = pixel_to_ndc(win_width, win_height, 235.0, 40.0);

    let vertices: [f32; 12] = [
        top_left.0,
        top_left.1,
        top_right.0,
        top_right.1,
        bottom_right.0,
        bottom_right.1,
        bottom_right.0,
        bottom_right.1,
        bottom_left.0,
        bottom_left.1,
        top_left.0,
        top_left.1,
    ];

    unsafe {
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
    }
}
impl State {
    fn new(gl: &glow::Context) -> Self {
        let win_width = 800.0;
        let win_height = 600.0;
        let vao = gen_vao(win_width, win_height, gl);

        State {
            win_width,
            win_height,
            vao,
        }
    }

    fn regen_vao(&mut self, gl: &glow::Context) {
        self.vao = gen_vao(self.win_width, self.win_height, gl);
    }
}

fn pixel_to_ndc(win_width: f32, win_height: f32, x: f32, y: f32) -> (f32, f32) {
    let x_ratio = 2.0 / win_width;
    let y_ratio = 2.0 / win_height;

    (-1.0 + (x * x_ratio), 1.0 - (y * y_ratio))
}

fn main() {
    let context = create_sdl2_context();

    let gl = context.0;
    let window = context.1;
    let mut events_loop = context.2;

    let mut state = State::new(&gl);

    let shader = shader::Shader::build(&gl, "res/shaders/basic.vert", "res/shaders/basic.frag");
    shader.activate(&gl);

    let _interval = window.subsystem().gl_set_swap_interval(1);

    'render: loop {
        let should_close = handle_events(&gl, &mut events_loop, &mut state);

        if should_close {
            break 'render;
        }

        unsafe {
            gl.clear_color(1.0, 1.0, 1.0, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            shader.activate(&gl);
            // shader.set_vec2(&gl, "dim", (2.0 / state.win_width, 2.0 / state.win_height));

            state.regen_vao(&gl);
            gl.bind_vertex_array(Some(state.vao));
            gl.draw_arrays(glow::TRIANGLES, 0, 6);
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

        let mut window = video
            .window("Browser", 800, 600)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let _a = window.set_minimum_size(300, 300);
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
                    state.regen_vao(gl);
                    gl.viewport(0, 0, x, y);
                },

                _ => {}
            }
        }
    }

    false
}
