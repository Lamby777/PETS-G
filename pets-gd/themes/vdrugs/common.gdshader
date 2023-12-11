shader_type canvas_item;

uniform float scroll_speed = 0.08;
//uniform vec2 texture_scroll = vec2(1.0, 0.0);

void fragment() {
    vec2 uv = UV;
    uv.x = UV.x * 1.0 + pow(abs(sin(PI * UV.x/2.0)),2.0);
    
    //COLOR = texture(TEXTURE, uv + texture_scroll);
    COLOR = texture(TEXTURE, uv + (scroll_speed * TIME));
}
