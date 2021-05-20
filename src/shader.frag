#version 450

layout(location=0) out vec4 f_color;
layout(location=0) in vec2 texcoords;

layout(set=0, binding=0)
uniform Uniforms {
    float u_time;
    vec2 u_resolution;
};

float plot(vec2 st, float pct) {
    return smoothstep( pct - 0.02, pct, st.y) -
           smoothstep( pct, pct + 0.02, st.y);
}

float quadraticBezier (float x, float a, float b){
  // adapted from BEZMATH.PS (1993)
  // by Don Lancaster, SYNERGETICS Inc. 
  // http://www.tinaja.com/text/bezmath.html

  float epsilon = 0.00001;
  a = max(0, min(1, a)); 
  b = max(0, min(1, b)); 
  if (a == 0.5){
    a += epsilon;
  }
  
  // solve t from x (an inverse operation)
  float om2a = 1 - 2*a;
  float t = (sqrt(a*a + om2a*x) - a)/om2a;
  float y = (1-2*b)*(t*t) + (2*b)*t;
  return y;
}

void main() {
    //f_color = vec4(abs(sin(2.0 * u_time) + 1) / 2.0, 0.0, 0.0, 1.0);
    //vec2 st = gl_FragCoord.xy / u_resolution;
    //f_color = vec4(vec3(plot(st, st.x), st.x, st.y), 1.0);
    //vec3 color = vec3(step(0.5, st.x), 0.1, step(0.5, st.y));
    vec2 st = texcoords.xy;

    float y = quadraticBezier(st.x, 0.99, 0.1);
    //(sin((50.0 * st.x) + u_time) + 1.0) / 2.0;
    
    vec3 color = vec3(y);

    float pct = plot(st, y);
    color = vec3(1.0-pct)*color + vec3(pct)*vec3(0.0, 1.0, 1.0);

    f_color = vec4(color, 1.0);
}