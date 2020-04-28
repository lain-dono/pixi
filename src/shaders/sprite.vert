#version 450

out gl_PerVertex {
    vec4 gl_Position;
};

layout(location = 0) out Vertex {
    vec2 tex_coord;
} vertex;

layout(location = 0) in vec2 a_Position;
layout(location = 1) in vec2 a_TexCoord;

layout(set = 0, binding = 0) uniform Globals {
    mat3 transform;
} globals;

void main() {
    vertex.tex_coord = a_TexCoord;
    gl_Position = vec4((globals.transform * vec3(a_Position, 1.0)).xy, 0.0, 1.0);
}