shader_type canvas_item;

void fragment() {
    // vec2 uv = UV;
    float v = sin(TIME) / 2.0 + 0.5;
    COLOR = vec4(v, v, v, 1.);
}

void vertex() {
  VERTEX += vec2(100, 100);
}
