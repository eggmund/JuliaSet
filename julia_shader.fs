#version 330

// Input vertex attributes (from vertex shader)

uniform vec2 screenDims;        // Dimensions of the screen
uniform vec2 c;                 // c.x = real, c.y = imaginary component. Equation done is z^2 + c
uniform vec2 offset;            // Offset of the scale.
uniform float zoom;             // Zoom of the scale.

// Output fragment color
out vec4 finalColor;

const int MAX_ITERATIONS = 512; // Max iterations to do.

// Square a complex number
vec2 complexSquare(vec2 z) {
   return vec2(
      z.x * z.x - z.y * z.y,
      z.x * z.y * 2.0
   );
}

// Convert Hue Saturation Value color into RGB
vec3 hsv2rgb(vec3 c) {
   vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
   vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
   return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}


void main() {
   vec2 z = vec2(((gl_FragCoord.x + offset.x)/screenDims.x) * 2.5/zoom,
                 ((screenDims.y - gl_FragCoord.y + offset.y)/screenDims.y) * 1.5/zoom); // y also flipped due to opengl
   int iterations = 0;

   for (iterations = 0; iterations < MAX_ITERATIONS; iterations++) {
      z = complexSquare(z) + c;
      if (dot(z, z) > 4.0) {
         break;
      }
   }
    
   z = complexSquare(z) + c;
   z = complexSquare(z) + c;
    
   float smoothVal = float(iterations) + 1.0 - (log(log(length(z)))/log(2.0));
    
   float norm = smoothVal/float(MAX_ITERATIONS);

   if (norm > 0.99999) {
      finalColor = vec4(0.0, 0.0, 0.0, 1.0);
   } else {
      finalColor = vec4(hsv2rgb(vec3(norm, 1.0, 1.0)), 1.0);
   }
}
