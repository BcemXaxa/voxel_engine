#version 450

layout(location = 0) out vec4 outColor;

void main() {
    vec2 c = vec2(gl_FragCoord.x / 400 - 2.7, gl_FragCoord.y / 400 - 1.25);

    vec2 z = vec2(0.0, 0.0);
    float i;
    for (i = 0.0; i < 1.0; i += 0.001) {
        z = vec2(
            z.x * z.x - z.y * z.y + c.x,
            z.y * z.x + z.x * z.y + c.y
        );

        if (length(z) > 4.0) {
            break;
        }
    }

    outColor = vec4(vec3(i), 1.0);
}