shader_type canvas_item;

uniform sampler2D SCREEN_TEXTURE: hint_screen_texture, filter_linear_mipmap;
uniform float SPEED = 0.25;
uniform float start_time = 0.;

float faketime() {
  return mod(TIME, 1.5); // loop for debugging purposes
}

float current_time() {
  return faketime() - start_time;
}

// between -1.0 and 1.0
float random(vec2 uv) {
  float r = fract(sin(dot(uv.xy, vec2(12.9898,78.233))) * 43758.5453123);
  return (r*2.) - 1.;
}

vec2 rand_modulation(float f) {
  float r1 = random(vec2(f, 1.));
  float r2 = random(vec2(f, 2.));
  return vec2(r1, r2); 
}

void fragment() {
  float progress = current_time();
  vec2 offset = vec2(SPEED, SPEED) * rand_modulation(TIME) * progress;
  COLOR = texture(SCREEN_TEXTURE, SCREEN_UV + offset);
}