#version 450

vec2 positions[3] = vec2[](
    vec2(-1, -1),
    vec2(1, -1),
    vec2(1, 1)
);
vec2 positions2[3] = vec2[](
    vec2(-1, -1),
    vec2(1, 1),
    vec2(-1, 1)
);

void main() {
    if(gl_VertexIndex < 3) {
        gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
    } else {
        gl_Position = vec4(positions2[gl_VertexIndex % 3], 0.0, 1.0);
    }
}