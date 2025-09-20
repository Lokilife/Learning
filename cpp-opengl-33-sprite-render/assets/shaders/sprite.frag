#version 330 core

in vec2 UV;

uniform sampler2D sprite;

void main() {
    gl_FragColor = texture(sprite, UV);
}
