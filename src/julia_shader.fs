#version 330
#extension GL_ARB_explicit_uniform_location : enable

layout(location = 0) uniform vec2 screen_dims;
layout(location = 1) uniform int max_iterations;
layout(location = 2) uniform vec2 c;  // c.x = real, c.y = imagin
layout(location = 3) uniform vec2 offset;
layout(location = 4) uniform float zoom;



// Output fragment color
out vec4 final_color;

vec2 complex_square(vec2 z) {
   return vec2(
      z.x * z.x - z.y * z.y,
      z.x * z.y * 2.0
   );
}

vec3 hsv2rgb(vec3 c) {
   vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
   vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
   return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}


vec4 get_color_bluepink(float norm) {
   norm = pow(norm, 1.0/6.0);
   return vec4(norm, norm/3.0, (norm/2.0) + 1.0, 1.0);
}

vec4 get_color_pastelgreen(float norm) {
   norm = pow(norm, 1.0/2.0);
   return vec4(norm/3.0, norm, norm/2.0, 1.0);
}


vec4 get_color_darkbluegrey(float norm) {
   norm = pow(norm, 1.0/2.0);
   return vec4(norm/3.0, norm/2.0, norm/2.0, 1.0);
}

vec4 get_color_turqoise(float norm) {
   norm = pow(norm, 1.0/2.0);
   return vec4(norm/1.5, norm, norm, 1.0);
}

vec4 get_color_blue(float norm) {
   float normsqrt = pow(norm, 1.0/2.0);
   return vec4(norm*norm, norm, normsqrt, 1.0);
}

vec4 get_color_rust(float norm) {
   float normsqrt = pow(norm, 1.0/2.0);
   return vec4(normsqrt, norm, norm*norm, 1.0);
}

vec4 get_color_red2blue(float norm) {
   return vec4(hsv2rgb(vec3(norm, 1.0, 1.0)), 1.0);
}

float julia() { // Returns between 0 and 1
   vec2 z = vec2((((gl_FragCoord.x + offset.x)/screen_dims.x) * 2.5) * zoom, (((screen_dims.y - gl_FragCoord.y + offset.y)/screen_dims.y) * 1.5) * zoom);
   int iterations = 0;
   
   for (iterations = 0; iterations < max_iterations; iterations++) {
      z = complex_square(z) + c;
      if (dot(z, z) > 4.0) { // zx * zx + zy * zy is dot product
         break;
      }
   }
   
   z = complex_square(z) + c;
   return float(iterations) + 1.0 - (log(log(length(z)))/log(2.0));
}

void main() {
   final_color = get_color_red2blue(julia()/float(max_iterations));
}
