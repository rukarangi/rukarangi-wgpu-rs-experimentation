#version 450

//layout(location=0) in vec3 position;
layout(location=0) out vec2 outUV;

// const vec2 positions[3] = vec2[3](
//     vec2(0.0, 0.5),
//     vec2(-0.5, -0.5),
//     vec2(0.5, -0.5)
// );

void main() {
    outUV = vec2((gl_VertexIndex << 1) & 2, gl_VertexIndex & 2);

    gl_Position = vec4(outUV * 2.0 + -1.0, 0.0, 1.0);

//    gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
}