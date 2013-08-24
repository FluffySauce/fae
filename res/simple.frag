#version 150
in vec3 Col;
out vec4 outCol;

void main() {
    outCol = vec4(Col, 1.0);
}

