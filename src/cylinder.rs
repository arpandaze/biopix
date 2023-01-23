use crate::object::Object;
use std::f32;

#[derive(Debug, Clone)]
pub struct Cylinder {
    pub vertices: Vec<f32>,
    pub normal_vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub colors: Vec<f32>,
    pub centre: [f32; 3],

    color: [f32; 3],
    sector_count: u32,
    radius: f32,
    height: f32,
    base_center_index: u32,
    top_center_index: u32,
    interlaced_vertices: Vec<f32>,
}

impl Cylinder {
    pub fn new(radius: f32, height: f32, sector_count: u32, color: [f32; 3]) -> Cylinder {
        let mut cyl = Cylinder {
            vertices: vec![],
            normal_vertices: vec![],
            indices: vec![],
            colors: vec![],
            centre: [0.0, 0.0, 0.0],
            base_center_index: 0,
            top_center_index: 0,
            color,
            sector_count,
            radius,
            height,
            interlaced_vertices: vec![],
        };

        cyl.generate_vertices();
        cyl.generate_indices();
        cyl.interlaced_vertices_generator();

        return cyl;
    }

    fn get_unit_circle_vertices(&self) -> Vec<f32> {
        let sector_step = 2.0 * std::f32::consts::PI / self.sector_count as f32;
        let mut unit_circle_vertices = Vec::new();

        for i in 0..=self.sector_count {
            let sector_angle = i as f32 * sector_step;
            unit_circle_vertices.push(sector_angle.cos());
            unit_circle_vertices.push(sector_angle.sin());
            unit_circle_vertices.push(0.0);
        }

        return unit_circle_vertices;
    }

    fn generate_vertices(&mut self) -> () {
        // Get the unit circle vertices on the XY-plane
        let unit_vertices = self.get_unit_circle_vertices();

        // Put side vertices into the arrays
        for i in 0..2 {
            let h = -self.height / 2.0 + i as f32 * self.height; // z value; -h/2 to h/2
            let t = 1.0 - i as f32; // vertical tex coord; 1 to 0

            for j in 0..=self.sector_count {
                let k = j * 3;
                let ux = unit_vertices[k as usize];
                let uy = unit_vertices[k as usize + 1];
                let uz = unit_vertices[k as usize + 2];

                // Position vector
                self.vertices.push(ux as f32 * self.radius); // vx
                self.vertices.push(uy as f32 * self.radius); // vy
                self.vertices.push(h); // vz
                self.colors.append(&mut self.color.to_vec());

                // Normal vector
                self.normal_vertices.push(ux); // nx
                self.normal_vertices.push(uy); // ny
                self.normal_vertices.push(uz); // nz
            }
        }

        self.base_center_index = self.vertices.len() as u32 / 3;
        self.top_center_index = self.base_center_index + self.sector_count + 1; // include center vertex

        for i in 0..2 {
            let h = -self.height / 2.0 + i as f32 * self.height; // z value; -h/2 to h/2
            let nz = -1.0 + i as f32 * 2.0; // z value of normal; -1 to 1

            // center point
            self.vertices.push(0.0);
            self.vertices.push(0.0);
            self.vertices.push(h);
            self.colors.append(&mut self.color.to_vec());
            self.normal_vertices.push(0.0);
            self.normal_vertices.push(0.0);
            self.normal_vertices.push(nz);

            for j in 0..self.sector_count {
                let k = j * 3;
                let ux = unit_vertices[k as usize];
                let uy = unit_vertices[k as usize + 1];
                self.vertices.push(ux * self.radius); // vx
                self.vertices.push(uy * self.radius); // vy
                self.vertices.push(h); // vz
                self.colors.append(&mut self.color.to_vec());
                self.normal_vertices.push(0.0); // nx
                self.normal_vertices.push(0.0); // ny
                self.normal_vertices.push(nz); // nz
            }
        }
    }

    fn generate_indices(&mut self) -> () {
        let mut k1 = 0;
        let mut k2 = self.sector_count + 1;

        for _ in 0..self.sector_count {
            // k1 => k1+1 => k2
            self.indices.push(k1);
            self.indices.push(k1 + 1);
            self.indices.push(k2);

            // k2 => k1+1 => k2+1
            self.indices.push(k2);
            self.indices.push(k1 + 1);
            self.indices.push(k2 + 1);

            k1 += 1;
            k2 += 1;
        }

        let mut k = self.base_center_index + 1;
        for i in 0..self.sector_count {
            if i < self.sector_count - 1 {
                self.indices.push(self.base_center_index);
                self.indices.push(k + 1);
                self.indices.push(k);
            } else {
                // last triangle
                self.indices.push(self.base_center_index);
                self.indices.push(self.base_center_index + 1);
                self.indices.push(k);
            }
            k += 1;
        }

        // indices for the top surface
        let mut k = self.top_center_index + 1; // FIXME: THis must be k++
        for i in 0..self.sector_count {
            if i < self.sector_count - 1 {
                self.indices.push(self.top_center_index);
                self.indices.push(k);
                self.indices.push(k + 1);
            } else {
                // last triangle
                self.indices.push(self.top_center_index);
                self.indices.push(k);
                self.indices.push(self.top_center_index + 1);
            }
            k += 1;
        }
    }

    fn interlaced_vertices_generator(&mut self) {
        self.interlaced_vertices = self
            .vertices()
            .chunks(3)
            .zip(self.normal_vertices().chunks(3))
            .zip(self.colors().chunks(3))
            .flat_map(|(a, b)| a.0.into_iter().chain(a.1).chain(b))
            .copied()
            .collect::<Vec<f32>>();
    }

    pub fn align_axis(&mut self, centre1: [f32; 3], centre2: [f32; 3]) {
        let prev_a = [0.0, 0.0, self.height / 2.0];
        let prev_b = [0.0, 0.0, -self.height / 2.0];

        let translation = [
            (centre1[0] + centre2[0]) / 2.0,
            (centre1[1] + centre2[1]) / 2.0,
            (centre1[2] + centre2[2]) / 2.0,
        ];

        let prev_vector = [
            prev_b[0] - prev_a[0],
            prev_b[1] - prev_a[1],
            prev_b[2] - prev_a[2],
        ];

        let new_vector = [
            centre2[0] - centre1[0],
            centre2[1] - centre1[1],
            centre2[2] - centre1[2],
        ];

        let prev_length =
            (prev_vector[0].powi(2) + prev_vector[1].powi(2) + prev_vector[2].powi(2)).sqrt();
        let new_length =
            (new_vector[0].powi(2) + new_vector[1].powi(2) + new_vector[2].powi(2)).sqrt();

        let target = [
            centre1[0] - translation[0],
            centre1[1] - translation[1],
            centre1[2] - translation[2],
        ];

        let target_hyp = (target[1].powi(2) + target[2].powi(2)).sqrt();
        let target_theta = (target[1] / target_hyp).asin();

        let target_xyz_hyp = (target[0].powi(2) + target_hyp.powi(2)).sqrt();
        let target_alpha = (target[0] / target_xyz_hyp).asin();

        let scale = new_length / prev_length;

        self.scale(1.0, 1.0, scale);
        self.rotate(0.0, target_alpha, 0.0);
        self.rotate(-target_theta, 0.0, 0.0);
        self.translate(translation[0], translation[1], translation[2]);
    }
}

impl Object for Cylinder {
    fn indices(&self) -> &Vec<u32> {
        return &self.indices;
    }

    fn vertices(&self) -> &Vec<f32> {
        return &self.vertices;
    }

    fn vertices_mut(&mut self) -> &mut Vec<f32> {
        return &mut self.vertices;
    }

    fn centre(&self) -> &[f32; 3] {
        return &self.centre;
    }

    fn centre_mut(&mut self) -> &mut [f32; 3] {
        return &mut self.centre;
    }

    fn colors(&self) -> &Vec<f32> {
        return &self.colors;
    }

    fn normal_vertices(&self) -> &Vec<f32> {
        return &self.normal_vertices;
    }

    fn interlaced_vertices(&self) -> &Vec<f32> {
        return &self.interlaced_vertices;
    }

    fn generate_interlaced_vertices(&mut self) {
        self.interlaced_vertices_generator();
    }
}
