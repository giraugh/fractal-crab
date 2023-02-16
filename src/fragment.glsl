#version 300 es

precision highp float;

uniform highp vec2 uResolution;
uniform highp vec2 uViewMin;
uniform highp vec2 uViewMax;

out vec4 outColor;

#define PI 3.141592653589793
#define FRAC_PI_3 1.0471975511965976
#define PI2_3 2.0943951023931953
#define MAX_ITS 1000.

vec3 sinebow(float x) {
  float t = (0.5 - x) * PI;
  return vec3(
    clamp(pow(sin(t), 2.0), 0.0, 1.0),
    clamp(pow(sin(t + FRAC_PI_3), 2.0), 0.0, 1.0),
    clamp(pow(sin(t + PI2_3), 2.0), 0.0, 1.0)
  );
}

float mandel(vec2 uv) {
  vec2 z = vec2(mix(uViewMin.x, uViewMax.x, 1.0 - uv.x), mix(uViewMin.y, uViewMax.y, uv.y));
  vec2 c = vec2(z);
  float its = 0.;
  
  while (true) {
    // Gone to inf?
    if (length(z) >= 3.) return its;
    
    // Out of time?
    its += 1.;
    if (its > MAX_ITS) return 0.;
    
    // perform op
    float x_t = (z.x*z.x) - (z.y*z.y) + c.x;
    z.y = (2. * z.x * z.y) + c.y;
    z.x = x_t;
  }
}

void main() {
  vec2 uv = gl_FragCoord.xy / uResolution;
  
  float its = mandel(uv) / 100.;
  vec3 col = its == 0. ? vec3(0.0) : sinebow(its);

  outColor = vec4(col, 1.0);
}
