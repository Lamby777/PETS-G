shader_type canvas_item;

uniform sampler2D SCREEN_TEXTURE: hint_screen_texture, filter_linear_mipmap;
uniform float SPEED = 0.25;
uniform float start_time = 0.;
uniform float current_time = 0.;
uniform vec2 rand_mod = vec2(0., 0.);

float elapsed() {
  return current_time - start_time;
}

void fragment() {
  vec2 offset = vec2(SPEED, SPEED) * rand_mod * elapsed();
  COLOR = texture(SCREEN_TEXTURE, SCREEN_UV + offset);
}