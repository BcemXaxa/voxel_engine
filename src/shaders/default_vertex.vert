#version 450

layout (location = 0) in vec2 pos;

layout (location = 1) in vec4 color;

layout (location = 0) out vec4 out_color;

// layout (std140) uniform Rotor {
//     mat3 rotation;
// } rotor;

void main() {
    gl_Position = vec4(pos, 0.0, 1.0);

    out_color = color;
}