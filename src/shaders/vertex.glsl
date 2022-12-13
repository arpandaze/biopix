#version 330
precision mediump float;
attribute vec3 position;
attribute vec3 color;
uniform float scale;
uniform mat4 rotation;
varying vec3 v_color;

uniform mat3 test = mat3(0.1, 0.0, 0.0, 0, 0.1, 0.0, 0, 0, 0.1, );

void main() {
  gl_Position = vec4(dot(test, position), 1.0);
  v_color = color;
}
