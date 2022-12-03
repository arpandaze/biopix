mod opengl;

use opengl::*;

// pub struct Sphere {
//     positions: Vec<[f32; 4]>,
//     indices: Vec<u16>,
// }
//
// impl Sphere {
//     pub fn new(slices: u16, stacks: u16) -> Sphere {
//         let size = 2 + slices * (stacks - 1);
//         let mut positions = Vec::with_capacity(4 * size as usize);
//
//         let alpha = std::f32::consts::PI / stacks as f32;
//         let beta = 2.0 * std::f32::consts::PI / slices as f32;
//
//         positions.push([0.0, 0.0, 1.0]);
//
//         for i in 1..stacks {
//             let i = i as f32;
//             for j in 0..slices {
//                 let j = j as f32;
//                 let r = (i * alpha).sin();
//                 let z = (i * alpha).cos();
//                 let y = (j * beta).sin() * r;
//                 let x = (j * beta).cos() * r;
//
//                 positions.push(Vertex {
//                     position: (x, y, z),
//                 });
//             }
//         }
//
//         positions.push(Vertex {
//             position: (0.0, 0.0, -1.0),
//         });
//
//         let mut indices: Vec<u16> =
//             Vec::with_capacity((3 * slices * (2 + 2 * (stacks - 2))) as usize);
//
//         for i in 1..slices {
//             indices.push(0);
//             indices.push(i);
//             indices.push(i + 1);
//         }
//         indices.push(0);
//         indices.push(slices);
//         indices.push(1);
//
//         for j in 1..stacks - 1 {
//             for i in 1..slices {
//                 indices.push(1 + (j - 1) * slices + i);
//                 indices.push(0 + (j - 1) * slices + i);
//                 indices.push(0 + j * slices + i);
//
//                 indices.push(0 + j * slices + i);
//                 indices.push(1 + j * slices + i);
//                 indices.push(1 + (j - 1) * slices + i);
//             }
//
//             indices.push(1 + (j - 1) * slices);
//             indices.push(slices + (j - 1) * slices);
//             indices.push(slices + j * slices);
//
//             indices.push(slices + j * slices);
//             indices.push(1 + j * slices);
//             indices.push(1 + (j - 1) * slices);
//         }
//
//         for i in 1..slices {
//             indices.push(size - 1);
//             indices.push(size - i - 1);
//             indices.push(size - i - 2);
//         }
//         indices.push(size - 1);
//         indices.push(size - slices - 1);
//         indices.push(size - 2);
//
//         Sphere {
//             positions: glium::VertexBuffer::new(facade, &positions).unwrap(),
//             indices: glium::IndexBuffer::new(
//                 facade,
//                 glium::index::PrimitiveType::TrianglesList,
//                 &indices,
//             )
//             .unwrap(),
//         }
//     }
//
//     pub fn get_positions(&self) -> &glium::VertexBuffer<Vertex> {
//         &self.positions
//     }
//     pub fn get_indices(&self) -> &glium::IndexBuffer<u16> {
//         &self.indices
//     }
// }

unsafe fn drawer(renderer: &mut opengl::Renderer) -> () {
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

    // renderer.gl.DeleteShader(vertex_shader);
    // renderer.gl.DeleteShader(fragment_shader);

    renderer.gl.GenVertexArrays(1, &mut renderer.vao);
    renderer.gl.BindVertexArray(renderer.vao);

    renderer.gl.GenBuffers(1, &mut renderer.vbo);
    renderer.gl.BindBuffer(gl::ARRAY_BUFFER, renderer.vbo);

    #[rustfmt::skip]
    let vertex_data: Vec<f32> = vec![
        -0.5, -0.5, 1.0, 1.0, 0.0, 0.0,
        0.5, -0.5, 1.0, 0.0, 1.0, 0.0,
        -0.5, 0.5, 1.0, 0.0, 0.0, 1.0,
        0.5, 0.5, 1.0, 1.0, 0.0, 0.0,
        -0.5, 0.5, 1.0, 0.0, 0.0, 1.0,
        0.5, -0.5, 1.0, 0.0, 1.0, 0.0,
    ];

    #[rustfmt::skip]
    let vertex_data2: Vec<f32> = vec![
        0.5, 0.5, 1.0, 1.0, 0.0, 0.0,
        -0.5, 0.5, 1.0, 0.0, 1.0, 0.0,
        0.5, -0.5, 1.0, 0.0, 0.0, 1.0,
    ];

    renderer.gl.BufferData(
        gl::ARRAY_BUFFER,
        (vertex_data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
        vertex_data.as_ptr() as *const _,
        gl::STATIC_DRAW,
    );

    let pos_attrib = renderer.gl.GetAttribLocation(
        renderer.program.unwrap(),
        b"position\0".as_ptr() as *const _,
    );

    let color_attrib = renderer
        .gl
        .GetAttribLocation(renderer.program.unwrap(), b"color\0".as_ptr() as *const _);

    renderer.gl.VertexAttribPointer(
        pos_attrib as gl::types::GLuint,
        3,
        gl::FLOAT,
        0,
        6 * std::mem::size_of::<f32>() as gl::types::GLsizei,
        std::ptr::null(),
    );

    renderer.gl.VertexAttribPointer(
        color_attrib as gl::types::GLuint,
        3,
        gl::FLOAT,
        0,
        6 * std::mem::size_of::<f32>() as gl::types::GLsizei,
        (3 * std::mem::size_of::<f32>()) as *const () as *const _,
    );

    renderer
        .gl
        .EnableVertexAttribArray(pos_attrib as gl::types::GLuint);

    renderer
        .gl
        .EnableVertexAttribArray(color_attrib as gl::types::GLuint);

    renderer.gl.ClearColor(0.1, 0.1, 0.1, 0.9);
    renderer.gl.Clear(gl::COLOR_BUFFER_BIT);
    renderer.gl.DrawArrays(gl::TRIANGLES, 0, 6);
}

const VERTEX_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;
attribute vec3 position;
attribute vec3 color;
varying vec3 v_color;
void main() {
    gl_Position = vec4(position, 1.0);
    v_color = color;
}
\0";

const FRAGMENT_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;
varying vec3 v_color;
void main() {
    gl_FragColor = vec4(v_color, 1.0);
}
\0";

pub fn main() {
    opengl::init(Some(drawer));
}
