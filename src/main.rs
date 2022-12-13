mod model;
mod opengl;
mod sphere;
mod scene;
mod object;

use model::Model;

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

    renderer.gl.GenBuffers(1, &mut renderer.vbo);
    renderer.gl.BindBuffer(gl::ARRAY_BUFFER, renderer.vbo);

    let vertex_data = &renderer.model.as_ref().unwrap().vertex;
    let vertex_indices = &renderer.model.as_ref().unwrap().index;

    let object = sphere::Sphere::new(100, 100, 2.0, [1.0, 0.0, 1.0]);

    // let vertex_data = object.vertices;
    // let vertex_indices = object.indices;

    let mut indices: gl::types::GLuint = std::mem::zeroed();

    renderer.gl.GenBuffers(1, &mut indices);
    renderer.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, indices);
    renderer.gl.BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (vertex_indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
        vertex_indices.as_ptr() as *const _,
        gl::STATIC_DRAW,
    );

    renderer.gl.BufferData(
        gl::ARRAY_BUFFER,
        (vertex_data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
        vertex_data.as_ptr() as *const _,
        gl::STATIC_DRAW,
    );

    // POSITION Attribute
    let pos_attrib = renderer.gl.GetAttribLocation(
        renderer.program.unwrap(),
        b"position\0".as_ptr() as *const _,
    );

    renderer.gl.VertexAttribPointer(
        pos_attrib as gl::types::GLuint,
        3,
        gl::FLOAT,
        0,
        6 * std::mem::size_of::<f32>() as gl::types::GLsizei,
        std::ptr::null(),
    );

    // COLOR Attribute
    let color_attrib = renderer
        .gl
        .GetAttribLocation(renderer.program.unwrap(), b"color\0".as_ptr() as *const _);

    renderer.gl.VertexAttribPointer(
        color_attrib as gl::types::GLuint,
        3,
        gl::FLOAT,
        0,
        6 * std::mem::size_of::<f32>() as gl::types::GLsizei,
        (3 * std::mem::size_of::<f32>()) as *const _,
    );

    // Scale Attribute
    let scale_attrib = renderer
        .gl
        .GetUniformLocation(renderer.program.unwrap(), b"scale\0".as_ptr() as *const _);
    renderer.gl.Uniform1f(scale_attrib, renderer.scale);

    // X Rotation Attribute
    let x_rotate_attrib = renderer.gl.GetUniformLocation(
        renderer.program.unwrap(),
        b"x_rotate\0".as_ptr() as *const _,
    );
    renderer
        .gl
        .Uniform1f(x_rotate_attrib, renderer.x_rotate.unwrap_or(0.0));

    // Y Rotation Attribute
    let y_rotate_attrib = renderer.gl.GetUniformLocation(
        renderer.program.unwrap(),
        b"y_rotate\0".as_ptr() as *const _,
    );
    renderer
        .gl
        .Uniform1f(y_rotate_attrib, renderer.y_rotate.unwrap_or(0.0));

    renderer
        .gl
        .EnableVertexAttribArray(pos_attrib as gl::types::GLuint);

    renderer
        .gl
        .EnableVertexAttribArray(color_attrib as gl::types::GLuint);

    renderer.gl.ClearColor(0.1, 0.1, 0.1, 0.9);
    renderer.gl.Clear(gl::COLOR_BUFFER_BIT);

    renderer.gl.Enable(gl::CULL_FACE);

    renderer.gl.DrawElements(
        gl::TRIANGLES,
        vertex_indices.len() as i32,
        gl::UNSIGNED_INT,
        0 as *const _,
    );
}

const VERTEX_SHADER_SOURCE: &[u8] = b"
precision mediump float;
attribute vec3 position;
attribute vec3 color;
uniform float scale;
uniform float x_rotate;
uniform float y_rotate;
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

// mat4 y_mat = mat4(
//     c_y, -s_y, 0, 0,
//     s_y, c_y, 0, 0,
//     0, 0, 1, 0,
//     0, 0, 0, 1
// );

void main() {
    gl_Position = y_mat * x_mat * vec4(scale * position, 1.0);
    v_color = color;
}
\0";

const FRAGMENT_SHADER_SOURCE: &[u8] = b"
precision mediump float;
varying vec3 v_color;

void main() {
    gl_FragColor = vec4(v_color * vec3(gl_FragCoord.z), 1.0);
}
\0";

pub fn main() {
    let model = Model::from("sphere.obj");

    opengl::init(drawer, Some(model));
}
