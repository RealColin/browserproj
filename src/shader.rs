use glow::*;

pub struct Shader {
    pub prog: NativeProgram,
}

impl Shader {
    pub fn build(gl: &glow::Context, vert_path: &str, frag_path: &str) -> Shader {
        let vert_shader = std::fs::read_to_string(vert_path).unwrap();
        let frag_shader = std::fs::read_to_string(frag_path).unwrap();

        unsafe {
            let program = gl.create_program().expect("Cannot create program");

            let shader_sources = [
                (glow::VERTEX_SHADER, vert_shader),
                (glow::FRAGMENT_SHADER, frag_shader),
            ];

            let mut shaders = Vec::with_capacity(shader_sources.len());

            for (shader_type, shader_source) in shader_sources.iter() {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect("Cannot create shader");
                gl.shader_source(shader, shader_source);
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    panic!("{}", gl.get_shader_info_log(shader));
                }
                gl.attach_shader(program, shader);
                shaders.push(shader);
            }

            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }

            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }

            let shader = Shader { prog: program };

            shader
        }
    }

    pub fn activate(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(Some(self.prog));
        }
    }
}
