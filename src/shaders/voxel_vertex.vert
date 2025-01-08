#version 450

layout (location = 0) in vec3 pos;

layout (location = 1) in vec4 color;

layout (location = 0) out vec4 out_color;

layout (push_constant) uniform Transform {
    mat4 pvm;
};

void main() {
    vec4 new_pos = pvm * vec4(pos, 1.0);
    gl_Position = new_pos;

    out_color = color;
}
