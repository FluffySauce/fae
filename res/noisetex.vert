#version 150
in vec2 pos;
in vec3 col;
in vec2 tex;
out vec3 Col;
out vec2 Tex;

void main() {
    Col = col;
    Tex = tex;
    gl_Position = vec4(pos, 0.0, 1.0);
}
