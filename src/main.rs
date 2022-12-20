mod cylinder;
mod model;
mod object;
mod opengl;
mod scene;
mod sphere;

use model::Model;
use object::Object;

use opengl::*;

unsafe fn drawer(renderer: &mut opengl::Renderer) {
    let vertex_shader =
        opengl::create_shader(&renderer.gl, gl::VERTEX_SHADER, VERTEX_SHADER_SOURCE);
    let fragment_shader = create_shader(&renderer.gl, gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE);

    renderer.program = Some(renderer.gl.CreateProgram());

    renderer
        .gl
        .AttachShader(renderer.program.unwrap(), vertex_shader);

    renderer
        .gl
        .AttachShader(renderer.program.unwrap(), fragment_shader);

    renderer.gl.LinkProgram(renderer.program.unwrap());

    renderer.gl.UseProgram(renderer.program.unwrap());

    renderer.gl.GenVertexArrays(1, &mut renderer.vao);
    renderer.gl.BindVertexArray(renderer.vao);
    //
    renderer.gl.GenBuffers(1, &mut renderer.vbo);
    renderer.gl.BindBuffer(gl::ARRAY_BUFFER, renderer.vbo);

    // renderer.gl.Enable(gl::CULL_FACE);
    renderer.gl.Enable(gl::DEPTH_TEST);
    renderer.gl.DepthFunc(gl::LESS);

    renderer.gl.Clear(gl::COLOR_BUFFER_BIT);
    renderer.gl.Clear(gl::DEPTH_BUFFER_BIT);
    renderer.gl.ClearColor(0.1, 0.1, 0.1, 1.0);

    let sphere1 = sphere::Sphere::new(100, 100, 5.0, [1.0, 1.0, 1.0]);
    sphere1.drawer(renderer);
    // let mut sphere2 = sphere::Sphere::new(100, 100, 5.0, [1.0, 1.0, 1.0]);
    let cyl1 = cylinder::Cylinder::new(2.0, 10.0, 100, [1.0, 0.0, 0.0]);
    cyl1.drawer(renderer);
}

const VERTEX_SHADER_SOURCE: &[u8] = b"
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

float s_x = sin(y_rotate);
float c_x = cos(y_rotate);

float s_y = sin(-x_rotate);
float c_y = cos(-x_rotate);

mat4 x_mat = mat4(
    1, 0, 0, 0,
    0, c_x, -s_x, 0,
    0, s_x, c_x, 0,
    0, 0, 0, 1
);

mat4 y_mat = mat4(
    c_y, 0, -s_y, 0,
    0,   1,    0, 0,
    s_y, 0,  c_y, 0,
    0,   0,    0, 1
);

void main() {
    vec4 final_position = y_mat * x_mat * vec4(scale * position, 1.0);
    gl_Position = final_position;

    v_color = color;
    v_normal = normal;
    v_position = vec3(final_position);
}
\0";

const FRAGMENT_SHADER_SOURCE: &[u8] = b"
precision mediump float;

varying vec3 v_position;
varying vec3 v_normal;
varying vec3 v_color;

vec3 light_position = vec3(-10.0, -10.0, -10.0);
vec3 light_color = vec3(0.2, 0.2, 0.2);
vec3 ambient_color = vec3(0.0, 0.0, 0.0);
float shininess = 0.0;

void main()
{
    // Calculate ambient lighting
    vec3 ambient = v_color * 0.05;

    // Calculate diffuse lighting
    vec3 lightDirection = normalize(light_position - v_position);
    float diffuse = max(dot(v_normal, lightDirection), 0.0);
    vec3 diffuseColor = v_color * light_color * diffuse;

    // Calculate specular lighting
    vec3 viewDirection = normalize(-v_position);
    vec3 reflectDirection = reflect(-lightDirection, v_normal);
    float specular = pow(max(dot(viewDirection, reflectDirection), 0.0), shininess);
    vec3 specularColor = light_color * specular;

    // Combine ambient, diffuse, and specular lighting
    vec3 finalColor = ambient + diffuseColor + specularColor;

    gl_FragColor = vec4(finalColor, 1.0);
}
\0";

pub fn main() {
    let model = Model::from("monkey.obj");

    opengl::init(drawer, Some(model));
}
