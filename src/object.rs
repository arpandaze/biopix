use crate::gl;
use crate::gl::*;
use crate::opengl;

pub trait Object {
    fn vertices_mut(&mut self) -> &mut Vec<f32>;
    fn vertices(&self) -> &Vec<f32>;
    fn colors(&self) -> &Vec<f32>;
    fn normal_vertices(&self) -> &Vec<f32>;
    fn indices(&self) -> &Vec<u32>;
    fn generate_interlaced_vertices(&mut self);

    fn translate(&mut self, x: f32, y: f32, z: f32) {
        let vertices = self.vertices_mut();

        vertices.as_mut_slice().chunks_mut(3).for_each(|point| {
            point[0] += x;
            point[1] += y;
            point[2] += z;
        });

        self.generate_interlaced_vertices();
    }

    fn scale(&mut self, x: f32, y: f32, z: f32) {
        let vertices = self.vertices_mut();

        vertices.as_mut_slice().chunks_mut(3).for_each(|point| {
            point[0] *= x;
            point[1] *= y;
            point[2] *= z;
        });
        self.generate_interlaced_vertices();
    }

    fn interlaced_vertices(&self) -> &Vec<f32>;

    unsafe fn drawer(&self, renderer: &mut opengl::Renderer) {
        let vertices = self.interlaced_vertices();
        let indices = self.indices();

        renderer.gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        renderer.gl.BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
            indices.as_ptr() as *const _,
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
            9 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            std::ptr::null(),
        );

        // Normal Attribute
        let normal_attrib = renderer
            .gl
            .GetAttribLocation(renderer.program.unwrap(), b"normal\0".as_ptr() as *const _);

        renderer.gl.VertexAttribPointer(
            normal_attrib as gl::types::GLuint,
            3,
            gl::FLOAT,
            0,
            9 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            (3 * std::mem::size_of::<f32>()) as *const _,
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
            9 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            (6 * std::mem::size_of::<f32>()) as *const _,
        );

        renderer
            .gl
            .EnableVertexAttribArray(pos_attrib as gl::types::GLuint);

        renderer
            .gl
            .EnableVertexAttribArray(normal_attrib as gl::types::GLuint);

        renderer
            .gl
            .EnableVertexAttribArray(color_attrib as gl::types::GLuint);

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

        renderer.gl.DrawElements(
            gl::TRIANGLES,
            indices.len() as i32,
            gl::UNSIGNED_INT,
            0 as *const _,
        );
    }
}
