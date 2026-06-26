#version 330

uniform vec4 beam_color;

void main() {
    // flat fill: no edge gradient, the whole band is one solid color
    gl_FragColor = beam_color;
}
