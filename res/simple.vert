#version 150
in vec2 pos;
in vec3 col;
out vec3 Col;

void main() {
    Col = col;
    gl_Position = vec4(pos, 0.0, 1.0);
}
