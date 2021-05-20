#version 450

//layout(location=0) in vec3 position;
layout(location=0) out vec2 texcoords;

// const vec2 positions[3] = vec2[3](
//     vec2(0.0, 0.5),
//     vec2(-0.5, -0.5),
//     vec2(0.5, -0.5)
// );

void main() {
    // outUV = vec2((gl_VertexIndex << 1) & 2, gl_VertexIndex & 2);

    // vec2 pos = vec2(outUV.x, outUV.y) * vec2(2.0, -2.0) + vec2(-1.0, 1.0);

    // gl_Position = vec4(pos, 0.0, 1.0);

//    gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);

    vec2 vertices[3] = vec2[3](vec2(-1, -1), vec2(3, -1), vec2(-1, 3));

    gl_Position = vec4(vertices[gl_VertexIndex], 0,1);

    texcoords = 0.5 * gl_Position.xy + vec2(0.5);
}