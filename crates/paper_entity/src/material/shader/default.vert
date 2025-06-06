#version 460 core

layout(location = 0) in vec3 aPos;
layout(location = 1) in vec4 aColor;

uniform mat4 uModel;

out vec4 fColor;

void main() {
    gl_Position = uModel * vec4(aPos, 1.0);
    fColor = aColor;
}
