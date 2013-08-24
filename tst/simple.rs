extern mod glfw;
extern mod gl;
extern mod fae;

use std::cast::transmute;
use std::path::PosixPath;
use std::sys::size_of;
use std::util::ignore;
use self::gl::types::*;
use fae::graphics::Shader;
use fae::graphics::Program;
use fae::graphics::BufferObject;
use fae::graphics::VertexArray;

static vertices: [GLfloat, ..20] = [
    -0.5,  0.5,     1.0, 0.0, 0.0,
     0.5,  0.5,     0.0, 1.0, 0.0,
     0.5, -0.5,     0.0, 0.0, 1.0,
    -0.5, -0.5,     1.0, 1.0, 0.0,
];

static indices: [GLubyte, ..6] = [
    0, 2, 3,
    0, 1, 2,
];

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}

fn main() {
    do glfw::set_error_callback |_, description| {
        printfln!("GLFW Error %s", description);
    }

    do glfw::start {
        glfw::window_hint::context_version(3, 2);
        glfw::window_hint::opengl_profile(glfw::OPENGL_CORE_PROFILE);
        glfw::window_hint::opengl_forward_compat(true);
        glfw::window_hint::resizable(false);

        let win = glfw::Window::create(512, 512, "Simple", glfw::Windowed).unwrap();
        win.make_context_current();
        gl::load_with(glfw::get_proc_address);

        let vert = Shader::load(gl::VERTEX_SHADER, &PosixPath("../res/simple.vert")).unwrap();
        let frag = Shader::load(gl::FRAGMENT_SHADER, &PosixPath("../res/simple.frag")).unwrap();
        let mut prog = Program::new();

        prog.attach(vert);
        prog.attach(frag);
        prog.bind_output(0, "outCol");
        ignore(prog.link());

        let posAttrib = prog.get_attrib("pos").unwrap();
        let colAttrib = prog.get_attrib("col").unwrap();
        
        let vao = VertexArray::new();
        let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW, vertices);
        let ibo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW, indices);

        vao.bind_attrib(vbo, posAttrib, 2, gl::FLOAT, 5 * size_of::<GLfloat>(), 0);
        vao.bind_attrib(vbo, colAttrib, 3, gl::FLOAT, 5 * size_of::<GLfloat>(), 2 * size_of::<GLfloat>());
        vao.bind_indices(ibo);

        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        let mut running = true;
        while running {
            glfw::poll_events();
            gl::Clear(gl::COLOR_BUFFER_BIT);

            prog.activate();
            vao.bind();
            unsafe {
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_BYTE, transmute(0));
            }

            win.swap_buffers();
            running = !win.should_close() && (win.get_key(glfw::KEY_ESCAPE) != glfw::PRESS);
        }

    }
}
