#version 450

layout(location=0) out vec4 f_color;
layout(location=0) in vec2 outUV;

layout(set=0, binding=0)
uniform Uniforms {
    float u_time;
    vec2 u_resolution;
};

void main() {
    //f_color = vec4(abs(sin(2.0 * u_time)), 0.0, 0.0, 1.0)
    vec2 st = gl_FragCoord.xy / u_resolution;
    
    float y = step(0.5, st.x);
    
    //f_color = vec4(st.x / 800.0, 0.0, 0.0, 1.0);
    f_color = vec4(vec3(y), 0.0);

}