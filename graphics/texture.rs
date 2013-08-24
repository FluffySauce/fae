extern mod gl;
use std::cast::transmute;
use std::vec::raw::to_ptr;
use self::gl::types::*;
use super::image::Image;

pub struct Texture {
    width:  uint,
    height: uint,
    id:     GLuint,
}

impl Texture {
    pub fn new(img: &Image) -> ~Texture {
        let t = ~Texture {
            width:  img.width,
            height: img.height,
            id:     0,
        };

        unsafe {
            gl::GenTextures(1, &t.id);
            gl::BindTexture(gl::TEXTURE_2D, t.id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as GLint,
                t.width as GLsizei, t.height as GLsizei,
                0, gl::RGBA, gl::UNSIGNED_BYTE,
                transmute(to_ptr(img.pixels)));
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
        }
        t
    }

    pub fn bind(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.id);
    }
}

impl Drop for Texture {
    fn drop(&self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
