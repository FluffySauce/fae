#version 150
in vec3 Col;
in vec2 Tex;
out vec4 outCol;

uniform sampler2D Texture;

void main() {
    outCol = texture(Texture, Tex) * vec4(Col, 1.0);
}

