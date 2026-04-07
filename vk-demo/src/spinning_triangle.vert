#version 450

layout(push_constant) uniform Push {
    float angle;
} pc;

layout(location = 0) out vec3 vColor;

vec2 positions[3] = vec2[](
    vec2(0.0, -0.6),
    vec2(0.6, 0.6),
    vec2(-0.6, 0.6)
);

vec3 colors[3] = vec3[](
    vec3(1.0, 0.0, 0.0),
    vec3(0.0, 1.0, 0.0),
    vec3(0.0, 0.0, 1.0)
);

void main() {
    vec2 p = positions[gl_VertexIndex];
    float c = cos(pc.angle);
    float s = sin(pc.angle);
    vec2 r = vec2(c * p.x - s * p.y, s * p.x + c * p.y);
    gl_Position = vec4(r, 0.0, 1.0);
    vColor = colors[gl_VertexIndex];
}
