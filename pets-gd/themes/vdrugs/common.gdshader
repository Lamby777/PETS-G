shader_type canvas_item;

void fragment() {
    float v = sin(TIME) / 2.0 + 0.5;
    COLOR = vec4(v, v, v, 1.0);
}
