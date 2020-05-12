#version 450

out gl_PerVertex {
    vec4 gl_Position;
};

layout(location = 0) in vec2 a_Position;

void main() {
    gl_Position = vec4(a_Position, 0.0, 1.0);
}