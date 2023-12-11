shader_type canvas_item;

void fragment() {
    // vec2 uv = UV;
    COLOR = vec4(1., abs(sin(TIME)), abs(sin(TIME)), 1.);
}
