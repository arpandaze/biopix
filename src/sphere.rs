use crate::object::Object;
use std::f32;

#[derive(Debug)]
pub struct Sphere {
    pub vertices: Vec<f32>,
    pub normal_vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub colors: Vec<f32>,

    color: [f32; 3],
    sector_count: u32,
    stack_count: u32,
    radius: f32,
}

impl Sphere {
    pub fn new(sector_count: u32, stack_count: u32, radius: f32, color: [f32; 3]) -> Sphere {
        let mut sphere = Sphere {
            vertices: vec![],
            normal_vertices: vec![],
            indices: vec![],
            colors: vec![],
            color,
            sector_count,
            stack_count,
            radius,
        };

        sphere.generate_vertices();
        sphere.generate_indices();

        return sphere;
    }

    fn generate_vertices(&mut self) -> () {
        let mut x;
        let mut y;
        let mut z;
        let mut xy;
        let mut nx;
        let mut ny;
        let mut nz;
        let length_inv = 1.0 / self.radius;

        let sector_step = 2.0 * f32::consts::PI / self.sector_count as f32;
        let stack_step = f32::consts::PI / self.stack_count as f32;
        let mut sector_angle;
        let mut stack_angle;

        for i in 0..=self.stack_count {
            stack_angle = (f32::consts::PI / 2.0) - i as f32 * stack_step;
            xy = self.radius * stack_angle.cos();
            z = self.radius * stack_angle.sin();

            for j in 0..=self.sector_count {
                sector_angle = j as f32 * sector_step;

                x = xy * sector_angle.cos();
                y = xy * sector_angle.sin();
                self.vertices.push(x);
                self.vertices.push(y);
                self.vertices.push(z);
                self.colors.append(&mut self.color.to_vec());

                nx = x * length_inv;
                ny = y * length_inv;
                nz = z * length_inv;
                self.normal_vertices.push(nx);
                self.normal_vertices.push(ny);
                self.normal_vertices.push(nz);
            }
        }
    }

    fn generate_indices(&mut self) -> () {
        for i in 0..self.stack_count {
            let mut k1 = i * (self.sector_count + 1);
            let mut k2 = k1 + self.sector_count + 1;

            for j in 0..self.sector_count {
                if i != 0 {
                    self.indices.push(k1);
                    self.indices.push(k2);
                    self.indices.push(k1 + 1);
                }

                if i != (self.stack_count - 1) {
                    self.indices.push(k1 + 1);
                    self.indices.push(k2);
                    self.indices.push(k2 + 1);
                }
                k1 = k1 + 1;
                k2 = k2 + 1;
            }
        }
    }
}

impl Object for Sphere {
    fn indices(&self) -> &Vec<u32> {
        return &self.indices;
    }

    fn vertices_mut(&mut self) -> &mut Vec<f32> {
        return &mut self.vertices;
    }

    fn vertices(&self) -> &Vec<f32> {
        return &self.vertices;
    }

    fn colors(&self) -> &Vec<f32> {
        return &self.colors;
    }

    fn normal_vertices(&self) -> &Vec<f32> {
        return &self.normal_vertices;
    }
}
