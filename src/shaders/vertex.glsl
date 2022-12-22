precision mediump float;
attribute vec3 position;
attribute vec3 normal;
attribute vec3 color;
uniform float scale;
uniform float x_rotate;
uniform float y_rotate;

varying vec3 v_position;
varying vec3 v_normal;
varying vec3 v_color;
varying mat4 light_dirn;

float s_x = sin(y_rotate);
float c_x = cos(y_rotate);

float s_y = sin(-x_rotate);
float c_y = cos(-x_rotate);

const float fovy = 90.0;
float aspect = 16.0 / 18.0;
float zNear = 0.1;
float zFar = 100.0;

mat4 x_mat = mat4(1, 0, 0, 0, 0, c_x, -s_x, 0, 0, s_x, c_x, 0, 0, 0, 0, 1);

mat4 y_mat = mat4(c_y, 0, -s_y, 0, 0, 1, 0, 0, s_y, 0, c_y, 0, 0, 0, 0, 1);

void main() {
  vec4 final_position = y_mat * x_mat * vec4(scale * position, 1.0);
  gl_Position = final_position;

  v_color = color;
  v_normal = normal;
  v_position = vec3(final_position);
  light_dirn = y_mat * x_mat;
}
