#version 330
#extension GL_ARB_explicit_uniform_location : enable

layout(location = 0) uniform vec2 screen_dims;
layout(location = 1) uniform int max_iterations;
layout(location = 2) uniform vec2 c;  // c.x = real, c.y = imagin
layout(location = 3) uniform vec2 offset;
layout(location = 4) uniform float zoom;



// Output fragment color
out vec4 final_color;

vec4 get_color_bluepink(float norm) {
   norm = pow(norm, 1.0/6.0);
   return vec4(norm, norm/3, (norm/2) + 127, 255);
}

vec4 get_color_pastelgreen(float norm) {
   norm = pow(norm, 1.0/6.0);
   return vec4(norm/3, norm, norm/2, 255);
}


vec4 get_color_darkbluegrey(float norm) {
   norm = pow(norm, 1.0/6.0);
   return vec4(norm/3, norm/2, norm/2, 255);
}

vec4 get_color_turqoise(float norm) {
   norm = pow(norm, 1.0/6.0);
   return vec4(norm/1.5, norm, norm, 255);
}


float julia() {
   float zx = (((gl_FragCoord.x + offset.x)/screen_dims.x) * 2.5) * zoom;
   float zy = (((screen_dims.y - gl_FragCoord.y + offset.y)/screen_dims.y) * 1.5) * zoom;
   int iterations = 0;
   
   while (zx * zx + zy * zy < 4.0 && iterations < max_iterations) {
      float xtemp = zx * zx - zy * zy;
      zy = 2.0 * zx * zy + c.y;
      zx = xtemp + c.x;

      iterations++;
   }

   return iterations;
}

void main() {
   final_color = get_color_turqoise(julia()/float(max_iterations));
}
