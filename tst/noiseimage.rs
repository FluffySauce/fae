extern mod glfw;
extern mod gl;
extern mod fae;

use std::cast::transmute;
use std::path::PosixPath;
use std::sys::size_of;
use std::util::ignore;
use self::gl::types::*;

use fae::graphics::BufferObject;
use fae::graphics::Color;
use fae::graphics::Image;
use fae::graphics::Program;
use fae::graphics::Shader;
use fae::graphics::Texture;
use fae::graphics::VertexArray;

use fae::noise::Clamp;
use fae::noise::Fbm;
use fae::noise::NoiseGen;
use fae::noise::Perlin;

static vertices: [GLfloat, ..28] = [
    -1.0,  1.0,     1.0, 0.0, 0.0,  0.0, 0.0,
     1.0,  1.0,     0.0, 1.0, 0.0,  1.0, 0.0,
     1.0, -1.0,     0.0, 0.0, 1.0,  1.0, 1.0,
    -1.0, -1.0,     1.0, 1.0, 0.0,  0.0, 1.0,
];

static indices: [GLubyte, ..6] = [
    0, 2, 3,
    0, 1, 2,
];

fn gen_tex(width: uint, height: uint) -> ~Texture {
    let p = Perlin::new(None);
    let f = Fbm::new(&p, 6, 1.0, 2.0, 0.8);
    let c = Clamp::new(&f, -1.0, 1.0);
    let mut img = Image::new(width, height);

    for y in range(0, height) {
        for x in range(0, width) {
            let xin = (x as float) / (width as float);
            let yin = (y as float) / (height as float);
            let res = (c.noise_2d(xin, yin) + 1.0) / 2.0;

            let c = (res * 255.0) as u8;
            img.set_pixel(x, y, Color::new(c, c, c, 255));
        }
    }

    Texture::new(&img)
}

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

        let win = glfw::Window::create(512, 512, "Noise Gen", glfw::Windowed).unwrap();
        win.make_context_current();
        gl::load_with(glfw::get_proc_address);

        let vert = Shader::load(gl::VERTEX_SHADER, &PosixPath("../res/noisetex.vert")).unwrap();
        let frag = Shader::load(gl::FRAGMENT_SHADER, &PosixPath("../res/noisetex.frag")).unwrap();
        let mut prog = Program::new();

        prog.attach(vert);
        prog.attach(frag);
        prog.bind_output(0, "outCol");
        ignore(prog.link());

        let posAttrib = prog.get_attrib("pos").unwrap();
        let colAttrib = prog.get_attrib("col").unwrap();
        let texAttrib = prog.get_attrib("tex").unwrap();
        let texUniform = prog.get_uniform("Texture").unwrap();

        let vao = VertexArray::new();
        let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW, vertices);
        let ibo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW, indices);

        vao.bind_attrib(vbo, posAttrib, 2, gl::FLOAT, size_of::<GLfloat>() * 7, 0);
        vao.bind_attrib(vbo, colAttrib, 3, gl::FLOAT, size_of::<GLfloat>() * 7, 2 * size_of::<GLfloat>());
        vao.bind_attrib(vbo, texAttrib, 2, gl::FLOAT, size_of::<GLfloat>() * 7, 5 * size_of::<GLfloat>());
        vao.bind_indices(ibo);

        let mut tex = gen_tex(512, 512);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        let mut running = true;
        while running {
            glfw::poll_events();

            if win.get_key(glfw::KEY_SPACE) == glfw::PRESS {
                tex = gen_tex(512, 512);
            }

            gl::Clear(gl::COLOR_BUFFER_BIT);

            prog.activate();
            vao.bind();

            gl::ActiveTexture(gl::TEXTURE0);
            tex.bind();
            gl::Uniform1i(texUniform as GLint, 0);

            unsafe {
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_BYTE, transmute(0));
            }

            win.swap_buffers();
            running = !win.should_close() && (win.get_key(glfw::KEY_ESCAPE) != glfw::PRESS);
        }
    }
}
