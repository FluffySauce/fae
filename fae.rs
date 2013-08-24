#[link(name = "fae",
       vers = "0.1",
       author = "FluffySauce",
       uuid = "BB2D6A25-D72E-4E67-9792-001C1E4F2F68")];
#[crate_type = "lib"];

pub mod graphics {
    pub use self::shader::*;
    pub use self::program::*;
    pub use self::bufferobject::*;
    pub use self::vertexarray::*;
    pub use self::color::*;
    pub use self::image::*;
    pub use self::texture::*;

    pub mod shader;
    pub mod program;
    pub mod bufferobject;
    pub mod vertexarray;
    pub mod color;
    pub mod image;
    pub mod texture;
}

pub mod noise {
    pub use self::noisegen::*;
    pub use self::perlin::*;
    pub use self::fbm::*;
    pub use self::clamp::*;

    pub mod noisegen;
    pub mod perlin;
    pub mod fbm;
    pub mod clamp;
}
