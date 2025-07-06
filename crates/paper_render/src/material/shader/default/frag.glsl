#version 460 core

out vec4 fragColor;

in vec4 fColor;

void main() {
    fragColor = fColor;
}
