#version 450

precision mediump float;

layout(location = 0) out vec4 target;

layout(location = 0) in VertexData {
    vec2 uv;
    vec4 color;
    uint id;
} vertex;

layout(set = 1, binding =  0) uniform sampler   S0;
layout(set = 1, binding =  1) uniform texture2D T0;
layout(set = 1, binding =  2) uniform sampler   S1;
layout(set = 1, binding =  3) uniform texture2D T1;
layout(set = 1, binding =  4) uniform sampler   S2;
layout(set = 1, binding =  5) uniform texture2D T2;
layout(set = 1, binding =  6) uniform sampler   S3;
layout(set = 1, binding =  7) uniform texture2D T3;
layout(set = 1, binding =  8) uniform sampler   S4;
layout(set = 1, binding =  9) uniform texture2D T4;
layout(set = 1, binding = 10) uniform sampler   S5;
layout(set = 1, binding = 11) uniform texture2D T5;
layout(set = 1, binding = 12) uniform sampler   S6;
layout(set = 1, binding = 13) uniform texture2D T6;
layout(set = 1, binding = 14) uniform sampler   S7;
layout(set = 1, binding = 15) uniform texture2D T7;
layout(set = 1, binding = 16) uniform sampler   S8;
layout(set = 1, binding = 17) uniform texture2D T8;
layout(set = 1, binding = 18) uniform sampler   S9;
layout(set = 1, binding = 19) uniform texture2D T9;
layout(set = 1, binding = 20) uniform sampler   S10;
layout(set = 1, binding = 21) uniform texture2D T10;
layout(set = 1, binding = 22) uniform sampler   S11;
layout(set = 1, binding = 23) uniform texture2D T11;
layout(set = 1, binding = 24) uniform sampler   S12;
layout(set = 1, binding = 25) uniform texture2D T12;
layout(set = 1, binding = 26) uniform sampler   S13;
layout(set = 1, binding = 27) uniform texture2D T13;
layout(set = 1, binding = 28) uniform sampler   S14;
layout(set = 1, binding = 29) uniform texture2D T14;
layout(set = 1, binding = 30) uniform sampler   S15;
layout(set = 1, binding = 31) uniform texture2D T15;

void main(void) {
    vec4 color;

         if (vertex.id ==  0) { color = texture(sampler2D( T0,  S0), vertex.uv); }
    else if (vertex.id ==  1) { color = texture(sampler2D( T1,  S1), vertex.uv); }
    else if (vertex.id ==  2) { color = texture(sampler2D( T2,  S2), vertex.uv); }
    else if (vertex.id ==  3) { color = texture(sampler2D( T3,  S3), vertex.uv); }
    else if (vertex.id ==  4) { color = texture(sampler2D( T4,  S4), vertex.uv); }
    else if (vertex.id ==  5) { color = texture(sampler2D( T5,  S5), vertex.uv); }
    else if (vertex.id ==  6) { color = texture(sampler2D( T6,  S6), vertex.uv); }
    else if (vertex.id ==  7) { color = texture(sampler2D( T7,  S7), vertex.uv); }
    else if (vertex.id ==  8) { color = texture(sampler2D( T8,  S8), vertex.uv); }
    else if (vertex.id ==  9) { color = texture(sampler2D( T9,  S9), vertex.uv); }
    else if (vertex.id == 10) { color = texture(sampler2D(T10, S10), vertex.uv); }
    else if (vertex.id == 11) { color = texture(sampler2D(T11, S11), vertex.uv); }
    else if (vertex.id == 12) { color = texture(sampler2D(T12, S12), vertex.uv); }
    else if (vertex.id == 13) { color = texture(sampler2D(T13, S13), vertex.uv); }
    else if (vertex.id == 14) { color = texture(sampler2D(T14, S14), vertex.uv); }
    else if (vertex.id == 15) { color = texture(sampler2D(T15, S15), vertex.uv); }

    target = color * vertex.color;
}