extern mod gl;
use std::cast::transmute;
use super::bufferobject::BufferObject;
use self::gl::types::*;

pub struct VertexArray {
    id: GLuint,
}

impl VertexArray {
    pub fn new() -> ~VertexArray {
        let v = ~VertexArray {
            id: 0,
        };
        unsafe {
            gl::GenVertexArrays(1, &v.id);
        }
        v
    }

    pub fn bind_attrib(&self, b: &BufferObject, attrib: GLuint, size: GLint, atype: GLenum,
        stride: uint, offset: uint) {
        gl::BindVertexArray(self.id);
        gl::BindBuffer(b.btype, b.id);
        gl::EnableVertexAttribArray(attrib);
        unsafe {
            gl::VertexAttribPointer(attrib, size, atype, gl::FALSE as GLboolean,
            stride as GLsizei, transmute(offset));
        }
    }

    pub fn bind_indices(&self, b: &BufferObject) {
        assert!(b.btype == gl::ELEMENT_ARRAY_BUFFER);
        gl::BindVertexArray(self.id);
        gl::BindBuffer(b.btype, b.id);
    }

    pub fn bind(&self) {
        gl::BindVertexArray(self.id);
    }
}

impl Drop for VertexArray {
    fn drop(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
