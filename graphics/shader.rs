extern mod gl;
use std::io::read_whole_file_str;
use std::path::Path;
use std::ptr::null;
use std::str::raw::from_bytes;
use std::vec::from_elem;
use std::vec::raw::to_ptr;
use self::gl::types::*;

pub struct Shader {
    stype:  GLenum,
    id:     GLuint,
}

impl Shader {
    pub fn new(stype: GLenum, source: &str) -> Result<~Shader, ~str> {
        let s = ~Shader {
            stype:  stype,
            id:     gl::CreateShader(stype),
        };

        unsafe {
            do source.with_c_str |sz| {
                gl::ShaderSource(s.id, 1, &sz, null());
            }
        }
        gl::CompileShader(s.id);

        let status: GLint = gl::FALSE as GLint;
        unsafe {
            gl::GetShaderiv(s.id, gl::COMPILE_STATUS, &status);
        }
        if status != (gl::TRUE as GLint) {
            unsafe {
                let len: GLint = 0;
                gl::GetShaderiv(s.id, gl::INFO_LOG_LENGTH, &len);
                let buff = from_elem(len as uint, 0u8);
                gl::GetShaderInfoLog(s.id, len, null(), to_ptr(buff) as *i8);
                Err(from_bytes(buff))
            }
        } else {
            Ok(s)
        }
    }

    pub fn load(stype: GLenum, path: &Path) -> Result<~Shader, ~str> {
        match read_whole_file_str(path) {
            Ok(contents) => Shader::new(stype, contents),
            Err(errstr) => Err(errstr),
        }
    }
}

impl Drop for Shader {
    fn drop(&self) {
        gl::DeleteShader(self.id);
    }
}

