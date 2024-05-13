shader_type canvas_item;

uniform float opacity : hint_range(0, 1) = 0.0;

void fragment() {
    COLOR.a = mix(0.0, 1.0, opacity);
}