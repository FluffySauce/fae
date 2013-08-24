extern mod gl;
use std::hashmap::HashMap;
use std::ptr::null;
use std::str::raw::from_bytes;
use std::vec::{from_elem, with_capacity};
use std::vec::raw::to_ptr;
use super::shader::Shader;
use self::gl::types::*;

pub struct Program {
    id:         GLuint,
    linked:     bool,
    shaders:    ~[~Shader],
    attribs:    HashMap<~str, GLint>,
    uniforms:   HashMap<~str, GLint>,
}

impl Program {
    pub fn new() -> ~Program {
        ~Program {
            id:         gl::CreateProgram(),
            linked:     false,
            shaders:    with_capacity(2),
            attribs:    HashMap::new(),
            uniforms:   HashMap::new(),
        }
    }

    pub fn attach(&mut self, s: ~Shader) {
        assert!(!self.linked);
        gl::AttachShader(self.id, s.id);
        self.shaders.push(s);
    }

    pub fn bind_output(&self, color: GLuint, name: &str) {
        assert!(!self.linked);
        unsafe {
            do name.with_c_str |sz| {
                gl::BindFragDataLocation(self.id, color, sz);
            }
        }
    }

    pub fn link(&mut self) -> Option<~str> {
        assert!(!self.linked);
        gl::LinkProgram(self.id);

        let status: GLint = gl::FALSE as GLint;
        unsafe {
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &status);
        }

        if status != (gl::TRUE as GLint) {
            unsafe {
                let len: GLint = 0;
                gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &len);
                let buff = from_elem(len as uint, 0u8);
                gl::GetProgramInfoLog(self.id, len, null(), to_ptr(buff) as *i8);
                Some(from_bytes(buff))
            }
        } else {
            self.linked = true;
            None
        }
    }

    pub fn get_attrib(&mut self, name: &str) -> Option<GLuint> {
        assert!(self.linked);
        let cached = *do self.attribs.find_or_insert_with(name.into_owned()) |cname| {
            unsafe {
                do cname.with_c_str |sz| {
                    gl::GetAttribLocation(self.id, sz)
                }
            }
        };

        match cached {
            _ if cached < 0 => None,
            loc => Some(loc as GLuint),
        }
    }

    pub fn get_uniform(&mut self, name: &str) -> Option<GLuint> {
        assert!(self.linked);
        let cached = *do self.uniforms.find_or_insert_with(name.into_owned()) |cname| {
            unsafe {
                do cname.with_c_str |sz| {
                    gl::GetUniformLocation(self.id, sz)
                }
            }
        };

        match cached {
            _ if cached < 0 => None,
            loc => Some(loc as GLuint),
        }
    }

    pub fn activate(&self) {
        assert!(self.linked);
        gl::UseProgram(self.id);
    }
}

impl Drop for Program {
    fn drop(&self) {
        for s in self.shaders.iter() {
            gl::DetachShader(self.id, s.id);
        }
        gl::DeleteProgram(self.id);
    }
}


