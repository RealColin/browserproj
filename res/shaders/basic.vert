#version 460 

uniform vec2 dim;

layout (location = 0) in vec2 aPos;

void main() {
    // gl_Position = vec4(-1.0 + (aPos.x * dim.x), 1.0 - (aPos.y * dim.y), 0.0, 1.0);
    gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0);
}
