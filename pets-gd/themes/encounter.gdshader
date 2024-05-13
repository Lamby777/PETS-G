shader_type canvas_item;

uniform sampler2D SCREEN_TEXTURE: hint_screen_texture, filter_linear_mipmap;

uniform bool DEBUG_MODE = false;
uniform float SPEED;
uniform float LENGTH;

uniform float start_time;
uniform float current_time;
uniform vec2 rand_mod;

void fragment() {
  float faketime = DEBUG_MODE ? mod(TIME, 1.) : current_time;
  
  float elapsed = faketime - start_time;
  float progress = min(elapsed / LENGTH, LENGTH);
  
  vec2 offset = vec2(SPEED, SPEED) * rand_mod * progress;
  vec4 col = texture(SCREEN_TEXTURE, SCREEN_UV + offset);

  col.rgb *= 1.0 - progress;
  
  COLOR = col;
}