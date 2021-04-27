#version 450

layout(location=0) out vec4 f_color;
layout(location=0) in vec2 outUV;

layout(set=0, binding=0)
uniform Uniforms {
    float u_time;
    vec2 u_resolution;
};

float plot(vec2 st) {
    return smoothstep(0.3, 0.1, abs(st.y - st.x));
}

void main() {
    //f_color = vec4(abs(sin(2.0 * u_time) + 1) / 2.0, 0.0, 0.0, 1.0);
    vec2 st = gl_FragCoord.xy / u_resolution;

    float y = st.x;
    
    vec3 color = vec3(y);

    float pct = smoothstep(0.3, 0.1, abs(st.y - st.x));

    color = (1.0-pct) * color + pct*vec3(0.0, 1.0, 1.0);  

    //f_color = vec4(st.x / 800.0, 0.0, 0.0, 1.0);
    f_color = vec4(color, 1.0);

}