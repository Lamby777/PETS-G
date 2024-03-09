shader_type canvas_item;

uniform sampler2D SCREEN_TEXTURE: hint_screen_texture, filter_linear_mipmap;
uniform float SPEED = 0.25;

float faketime() {
  return mod(TIME, 1.5); // loop for debugging purposes
}

void fragment() {
  COLOR = texture(SCREEN_TEXTURE, SCREEN_UV + faketime() * SPEED);
}