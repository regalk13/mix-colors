#ifdef GL_ES
precision highp float;
#endif

uniform vec2 u_resolution;
uniform vec2 u_mouse;
uniform float u_time;

uniform sampler2D mixbox_lut;

#include "mixbox.glsl"

void main() {
    vec2 uv = gl_FragCoord.xy / u_resolution;

    // Example colors (replace these with dynamic inputs later)
    vec3 rgb1 = vec3(0.0, 0.129, 0.522); // blue
    vec3 rgb2 = vec3(0.988, 0.827, 0.0); // yellow
    float t = length(uv - u_mouse / u_resolution); // Dynamic mix ratio based on distance to the mouse

    vec3 mixed_color = mixbox_lerp(rgb1, rgb2, t);

    gl_FragColor = vec4(mixed_color, 1.0);
}
