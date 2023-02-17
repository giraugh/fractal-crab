#version 300 es

precision highp float;

uniform highp vec2 uResolution;
uniform highp vec2 uViewCentreX; // These vec2s are actually floats but with emulated double precision
uniform highp vec2 uViewCentreY; // ^
uniform highp vec2 uViewSize; // ^

uniform bool uUseJuliaConstant;
uniform highp vec2 uJuliaConstant;

out vec4 outColor;

#define PI 3.141592653589793
#define FRAC_PI_3 1.0471975511965976
#define PI2_3 2.0943951023931953
#define MAX_ITS 1500.

vec3 sinebow(float x) {
  float t = (0.5 - x) * PI;
  return vec3(
    clamp(pow(sin(t), 2.0), 0.0, 1.0),
    clamp(pow(sin(t + FRAC_PI_3), 2.0), 0.0, 1.0),
    clamp(pow(sin(t + PI2_3), 2.0), 0.0, 1.0)
  );
}

/*
  ---- EHP ----
  Emulated double precision code adapted from a guide found here
  https://blog.cyclemap.link/2011-06-09-glsl-part2-emu/
*/

vec2 ehp_from_lp(float lp){
  vec2 ehp;
  ehp.x = lp;
  ehp.y = 0.0;
  return ehp;
}

float lp_from_ehp(vec2 ehp) {
  return ehp.x;
}

vec2 ehp_add(vec2 ehp1, vec2 ehp2) {
  vec2 ehp3;
  float t1, t2, e;

  t1 = ehp1.x + ehp2.x;
  e = t1 - ehp1.x;
  t2 = ((ehp2.x - e) + (ehp1.x - (t1 - e))) + ehp1.y + ehp2.y;

  ehp3.x = t1 + t2;
  ehp3.y = t2 - (ehp3.x - t1);
  return ehp3;
}

vec2 ehp_sub(vec2 ehp1, vec2 ehp2) {
  vec2 ehp3;
  float e, t1, t2;

  t1 = ehp1.x - ehp2.x;
  e = t1 - ehp1.x;
  t2 = ((-ehp2.x - e) + (ehp1.x - (t1 - e))) + ehp1.y - ehp2.y;

  ehp3.x = t1 + t2;
  ehp3.y = t2 - (ehp3.x - t1);
  return ehp3;
}

vec2 ehp_mul(vec2 ehp1, vec2 ehp2) {
  vec2 ehp3;
  float c11, c21, c2, e, t1, t2;
  float a1, a2, b1, b2, cona, conb, split = 8193.;

  cona = ehp1.x * split;
  conb = ehp2.x * split;
  a1 = cona - (cona - ehp1.x);
  b1 = conb - (conb - ehp2.x);
  a2 = ehp1.x - a1;
  b2 = ehp2.x - b1;

  c11 = ehp1.x * ehp2.x;
  c21 = a2 * b2 + (a2 * b1 + (a1 * b2 + (a1 * b1 - c11)));

  c2 = ehp1.x * ehp2.y + ehp1.y * ehp2.x;

  t1 = c11 + c2;
  e = t1 - c11;
  t2 = ehp1.y * ehp2.y + ((c2 - e) + (c11 - (t1 - e))) + c21;

  ehp3.x = t1 + t2;
  ehp3.y = t2 - (ehp3.x - t1);

  return ehp3;
}

// -1 lt
//  0 eq
//  1 gt
float ehp_cmp(vec2 ehp1, vec2 ehp2) {
  if (ehp1.x < ehp2.x) return -1.;
  else if (ehp1.x == ehp2.x) 
    {
      if (ehp1.y < ehp2.y) return -1.;
      else if (ehp1.y == ehp2.y) return 0.;
      else return 1.;
    }
  else return 1.;
}

vec2 ehp_mix(vec2 a, vec2 b, float t) {
  vec2 ehpt = ehp_from_lp(t);
  return ehp_add(a, ehp_mul(ehp_sub(b, a), ehpt));
}

/*
 ----- End -----
*/


float mandel(vec2 uv) {
  vec2 zx = ehp_mix(ehp_sub(uViewCentreX, uViewSize), ehp_add(uViewCentreX, uViewSize), uv.x);
  vec2 zy = ehp_mix(ehp_sub(uViewCentreY, uViewSize), ehp_add(uViewCentreY, uViewSize), uv.y);

  vec2 cx = uUseJuliaConstant ? ehp_from_lp(uJuliaConstant.x) : zx;
  vec2 cy = uUseJuliaConstant ? ehp_from_lp(uJuliaConstant.y) : zy;
  float its = 0.;

  while (true) {
    // Gone to inf?
    vec2 zmagsq = ehp_add(ehp_mul(zx, zx), ehp_mul(zy, zy));
    if (ehp_cmp(zmagsq, ehp_from_lp(9.)) > 0.) {
      return its;
    }

    // Out of time?
    its += 1.;
    if (its > MAX_ITS) return 0.;
  
    // Perform op
    vec2 xt = ehp_add(ehp_sub(ehp_mul(zx, zx), ehp_mul(zy, zy)), cx);
    zy = ehp_add(ehp_mul(ehp_mul(ehp_from_lp(2.), zx), zy), cy);
    zx = xt;
  }

  // vec2 z = vec2(
  //   mix(uViewCentreX - uViewSize, uViewCentreX + uViewSize, uv.x),
  //   mix(uViewCentreY - uViewSize, uViewCentreY + uViewSize, uv.y)
  // );
  // vec2 c = vec2(z);
  // float its = 0.;
  
  // while (true) {
  //   // Gone to inf?
  //   if (length(z) >= 3.) return its;
    
  //   // Out of time?
  //   its += 1.;
  //   if (its > MAX_ITS) return 0.;
    
  //   // perform op
  //   float x_t = (z.x*z.x) - (z.y*z.y) + c.x;
  //   z.y = (2. * z.x * z.y) + c.y;
  //   z.x = x_t;
  // }
}

void main() {
  vec2 uv = gl_FragCoord.xy / uResolution;
  
  float its = mandel(uv) / 100.;
  vec3 col = its == 0. ? vec3(0.0) : sinebow(its);

  outColor = vec4(col, 1.0);
}
