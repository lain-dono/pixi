#version 450

precision highp float;

layout(location = 0) in vec2 aPosition;
layout(location = 1) in vec2 aTexCoord;
layout(location = 2) in vec4 aColor;
layout(location = 3) in uint aTextureId;

layout(location = 0) out VertexData {
    vec2 uv;
    vec4 color;
    uint id;
} vertex;

layout(set = 0, binding = 0) uniform Globals0 {
    mat3 projection;
};

layout(set = 2, binding = 0) uniform Globals2 {
    mat3 translation;
    vec4 tint;
};

void main(void) {
    gl_Position = vec4((projection * translation * vec3(aPosition, 1.0)).xy, 0.0, 1.0);

    vertex.uv = aTexCoord;
    vertex.id = aTextureId;
    vertex.color = aColor * tint;
}