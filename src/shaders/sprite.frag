#version 450

precision mediump float;

layout(location = 0) out vec4 o_Target;

layout(location = 0) in Vertex {
    vec2 tex_coord;
} vertex;

layout(set = 1, binding = 0) uniform texture2D t_Color;
layout(set = 1, binding = 1) uniform sampler s_Color;

void main() {
    vec4 color = texture(sampler2D(t_Color, s_Color), vertex.tex_coord);
    o_Target = color;
}