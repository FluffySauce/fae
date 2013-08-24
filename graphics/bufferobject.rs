extern mod gl;
use std::cast::transmute;
use std::sys::size_of;
use std::vec::raw::to_ptr;
use self::gl::types::*;

pub struct BufferObject {
    btype:      GLenum,
    usage:      GLenum,
    id:         GLuint,
    elem_size:  uint,
    elem_count: uint,
    size:       uint,
}

impl BufferObject {
    pub fn new<T>(btype: GLenum, usage: GLenum, data: &[T]) -> ~BufferObject {
        let b = ~BufferObject {
            btype:      btype,
            usage:      usage,
            id:         0,
            elem_size:  size_of::<T>(),
            elem_count: data.len(),
            size:       size_of::<T>() * data.len(),
        };

        unsafe {
            gl::GenBuffers(1, &b.id);
            gl::BindBuffer(b.btype, b.id);
            gl::BufferData(b.btype, b.size as GLsizeiptr, transmute(to_ptr(data)), usage);
        }
        b
    }

    pub fn bind(&self) {
        gl::BindBuffer(self.btype, self.id);
    }
}

impl Drop for BufferObject {
    fn drop(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
